use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entity::refresh_token::RefreshToken;
use crate::domain::valueobject::email::Email;
use crate::domain::valueobject::jwt::{JwtError, JwtService, TokenPair};
use crate::repository::refresh_token_repository::{
    RefreshTokenRepository, RefreshTokenRepositoryError,
};
use crate::repository::user_repository::{UserRepository, UserRepositoryError};

/// 認証エラー
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Token revoked")]
    TokenRevoked,
    #[error("User not found")]
    UserNotFound,
    #[error("Repository error: {0}")]
    Repository(String),
    #[error("JWT error: {0}")]
    Jwt(String),
}

impl From<UserRepositoryError> for AuthError {
    fn from(err: UserRepositoryError) -> Self {
        Self::Repository(err.to_string())
    }
}

impl From<RefreshTokenRepositoryError> for AuthError {
    fn from(err: RefreshTokenRepositoryError) -> Self {
        Self::Repository(err.to_string())
    }
}

impl From<JwtError> for AuthError {
    fn from(err: JwtError) -> Self {
        match err {
            JwtError::Expired => Self::TokenExpired,
            JwtError::InvalidTokenType => Self::InvalidToken,
            _ => Self::Jwt(err.to_string()),
        }
    }
}

/// ログインリクエスト
#[derive(Debug, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// ログインレスポンス
#[derive(Debug, Clone)]
pub struct LoginResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub token_pair: TokenPair,
}

/// 認証サービス
pub struct AuthService<U: UserRepository, R: RefreshTokenRepository> {
    user_repository: Arc<U>,
    refresh_token_repository: Arc<R>,
    jwt_service: JwtService,
}

impl<U: UserRepository, R: RefreshTokenRepository> AuthService<U, R> {
    /// 新しい認証サービスを作成
    #[must_use]
    pub fn new(
        user_repository: Arc<U>,
        refresh_token_repository: Arc<R>,
        jwt_secret: String,
    ) -> Self {
        Self {
            user_repository,
            refresh_token_repository,
            jwt_service: JwtService::new(jwt_secret),
        }
    }

    /// ログイン処理：メール+パスワード → TokenPair + DB保存
    ///
    /// # Errors
    ///
    /// - 認証情報が無効な場合
    /// - DB操作に失敗した場合
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, AuthError> {
        // 1. メールアドレスのバリデーション
        let email = Email::new(&request.email).map_err(|_| AuthError::InvalidCredentials)?;

        // 2. ユーザーの取得
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        // 3. パスワードの検証
        let is_valid = user
            .verify_password(&request.password)
            .map_err(|_| AuthError::InvalidCredentials)?;
        if !is_valid {
            return Err(AuthError::InvalidCredentials);
        }

        // 4. Token Pairを生成
        let token_pair = self.jwt_service.generate_token_pair(user.user_id())?;

        // 5. Refresh TokenをDBに保存
        let token_hash = JwtService::hash_refresh_token(&token_pair.refresh_token);
        let refresh_token = RefreshToken::new(*user.user_id(), token_hash);
        self.refresh_token_repository.save(&refresh_token).await?;

        // 6. レスポンスを返す
        Ok(LoginResponse {
            user_id: user.user_id().to_string(),
            username: user.username().as_str().to_string(),
            email: user.email().as_str().to_string(),
            token_pair,
        })
    }

    /// トークンリフレッシュ：古いRefresh Token → 新しいTokenPair
    ///
    /// # Errors
    ///
    /// - Refresh Tokenが無効な場合
    /// - Refresh TokenがDBに存在しない、または無効化されている場合
    pub async fn refresh(&self, refresh_token: &str) -> Result<TokenPair, AuthError> {
        // 1. Refresh Tokenを検証
        let claims = self.jwt_service.verify_refresh_token(refresh_token)?;
        let user_id =
            Uuid::parse_str(&claims.sub).map_err(|_| AuthError::InvalidToken)?;

        // 2. DBでトークンを確認
        let token_hash = JwtService::hash_refresh_token(refresh_token);
        let stored_token = self
            .refresh_token_repository
            .find_by_hash(&token_hash)
            .await?
            .ok_or(AuthError::InvalidToken)?;

        // 3. トークンが有効か確認
        if !stored_token.is_valid() {
            return Err(AuthError::TokenRevoked);
        }

        // 4. 古いトークンを削除（Token Rotation）
        self.refresh_token_repository
            .delete_by_id(stored_token.token_id())
            .await?;

        // 5. 新しいToken Pairを生成
        let new_token_pair = self.jwt_service.generate_token_pair(&user_id)?;

        // 6. 新しいRefresh TokenをDBに保存
        let new_token_hash = JwtService::hash_refresh_token(&new_token_pair.refresh_token);
        let new_refresh_token = RefreshToken::new(user_id, new_token_hash);
        self.refresh_token_repository.save(&new_refresh_token).await?;

        Ok(new_token_pair)
    }

    /// ログアウト：Refresh Tokenを無効化
    ///
    /// # Errors
    ///
    /// - DB操作に失敗した場合
    pub async fn logout(&self, refresh_token: &str) -> Result<(), AuthError> {
        let token_hash = JwtService::hash_refresh_token(refresh_token);

        if let Some(stored_token) = self
            .refresh_token_repository
            .find_by_hash(&token_hash)
            .await?
        {
            self.refresh_token_repository
                .delete_by_id(stored_token.token_id())
                .await?;
        }

        Ok(())
    }

    /// 全デバイスからログアウト
    ///
    /// # Errors
    ///
    /// - DB操作に失敗した場合
    pub async fn logout_all(&self, user_id: &Uuid) -> Result<(), AuthError> {
        self.refresh_token_repository
            .delete_all_by_user_id(user_id)
            .await?;
        Ok(())
    }

    /// Access Tokenを検証してユーザーIDを取得
    ///
    /// # Errors
    ///
    /// - トークンが無効な場合
    pub fn validate_access_token(&self, token: &str) -> Result<Uuid, AuthError> {
        let claims = self.jwt_service.verify_access_token(token)?;
        Uuid::parse_str(&claims.sub).map_err(|_| AuthError::InvalidToken)
    }
}

#[cfg(test)]
mod tests {
    // テストは後で追加
}
