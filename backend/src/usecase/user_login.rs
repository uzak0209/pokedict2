use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::domain::valueobject::email::{Email, EmailValidationError};
use crate::domain::valueobject::hashedpassword::PasswordError;
use crate::repository::user_repository::{UserRepository, UserRepositoryError};

/// JWTのクレーム
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // ユーザーID
    pub username: String,
    pub email: String,
    pub exp: i64, // 有効期限（UNIX timestamp）
    pub iat: i64, // 発行時刻（UNIX timestamp）
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
    pub token: String,
}

/// ログイン時のエラー
#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Email validation failed: {0}")]
    EmailValidation(String),
    #[error("Repository error: {0}")]
    Repository(String),
    #[error("Token generation failed: {0}")]
    TokenGeneration(String),
}

impl From<EmailValidationError> for LoginError {
    fn from(err: EmailValidationError) -> Self {
        Self::EmailValidation(err.to_string())
    }
}

impl From<UserRepositoryError> for LoginError {
    fn from(err: UserRepositoryError) -> Self {
        Self::Repository(err.to_string())
    }
}

impl From<PasswordError> for LoginError {
    fn from(_: PasswordError) -> Self {
        // セキュリティのため、詳細を隠す
        Self::InvalidCredentials
    }
}

impl From<jsonwebtoken::errors::Error> for LoginError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::TokenGeneration(err.to_string())
    }
}

/// ユーザーログインのユースケース
pub struct UserLoginUseCase<R: UserRepository> {
    user_repository: Arc<R>,
    jwt_secret: String,
    token_expiration_hours: i64,
}

impl<R: UserRepository> UserLoginUseCase<R> {
    /// 新しいユースケースインスタンスを作成
    ///
    /// # Arguments
    ///
    /// * `user_repository` - ユーザーリポジトリ
    /// * `jwt_secret` - JWT署名用の秘密鍵
    /// * `token_expiration_hours` - トークンの有効期限（時間）
    #[must_use]
    pub fn new(user_repository: Arc<R>, jwt_secret: String, token_expiration_hours: i64) -> Self {
        Self {
            user_repository,
            jwt_secret,
            token_expiration_hours,
        }
    }

    /// ログイン処理を実行
    ///
    /// # Errors
    ///
    /// - メールアドレスのバリデーションエラー
    /// - ユーザーが見つからない場合
    /// - パスワードが一致しない場合
    /// - トークン生成に失敗した場合
    pub async fn execute(&self, request: LoginRequest) -> Result<LoginResponse, LoginError> {
        // 1. メールアドレスのバリデーション
        let email = Email::new(&request.email)?;

        // 2. ユーザーの取得
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or(LoginError::InvalidCredentials)?;

        // 3. パスワードの検証
        let is_valid = user.verify_password(&request.password)?;
        if !is_valid {
            return Err(LoginError::InvalidCredentials);
        }

        // 4. JWTトークンの生成
        let now = Utc::now();
        let expiration = now + Duration::hours(self.token_expiration_hours);

        let claims = Claims {
            sub: user.user_id().to_string(),
            username: user.username().as_str().to_string(),
            email: user.email().as_str().to_string(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        // 5. レスポンスを返す
        Ok(LoginResponse {
            user_id: user.user_id().to_string(),
            username: user.username().as_str().to_string(),
            email: user.email().as_str().to_string(),
            token,
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::domain::entity::user::User;
    use crate::domain::valueobject::username::Username;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use uuid::Uuid;

    // テスト用のインメモリリポジトリ
    struct InMemoryUserRepository {
        users: Mutex<HashMap<Uuid, User>>,
    }

    impl InMemoryUserRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
            }
        }

        fn add_user(&self, user: User) {
            let mut users = self.users.lock().unwrap();
            users.insert(*user.user_id(), user);
        }
    }

    #[async_trait]
    impl UserRepository for InMemoryUserRepository {
        async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
            let mut users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            users.insert(*user.user_id(), user.clone());
            Ok(())
        }

        async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, UserRepositoryError> {
            let users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            Ok(users.get(user_id).cloned())
        }

        async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError> {
            let users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            Ok(users.values().find(|u| u.email().as_str() == email.as_str()).cloned())
        }

        async fn find_by_username(
            &self,
            username: &Username,
        ) -> Result<Option<User>, UserRepositoryError> {
            let users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            Ok(users.values().find(|u| u.username().as_str() == username.as_str()).cloned())
        }

        async fn exists_by_email(&self, email: &Email) -> Result<bool, UserRepositoryError> {
            Ok(self.find_by_email(email).await?.is_some())
        }

        async fn exists_by_username(
            &self,
            username: &Username,
        ) -> Result<bool, UserRepositoryError> {
            Ok(self.find_by_username(username).await?.is_some())
        }
    }

    #[actix_rt::test]
    async fn test_login_success() {
        let repository = Arc::new(InMemoryUserRepository::new());

        // テスト用ユーザーを作成
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        let password = "password123";
        let user = User::new(username, email.clone(), password).unwrap();
        repository.add_user(user);

        let usecase = UserLoginUseCase::new(
            repository,
            "test_secret_key".to_string(),
            24, // 24時間有効
        );

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = usecase.execute(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.username, "testuser");
        assert_eq!(response.email, "test@example.com");
        assert!(!response.token.is_empty());
    }

    #[actix_rt::test]
    async fn test_login_wrong_password() {
        let repository = Arc::new(InMemoryUserRepository::new());

        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        let password = "password123";
        let user = User::new(username, email.clone(), password).unwrap();
        repository.add_user(user);

        let usecase = UserLoginUseCase::new(repository, "test_secret_key".to_string(), 24);

        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "wrong_password".to_string(),
        };

        let result = usecase.execute(request).await;
        assert!(matches!(result, Err(LoginError::InvalidCredentials)));
    }

    #[actix_rt::test]
    async fn test_login_user_not_found() {
        let repository = Arc::new(InMemoryUserRepository::new());
        let usecase = UserLoginUseCase::new(repository, "test_secret_key".to_string(), 24);

        let request = LoginRequest {
            email: "nonexistent@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = usecase.execute(request).await;
        assert!(matches!(result, Err(LoginError::InvalidCredentials)));
    }
}
