use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entity::refresh_token::RefreshToken;

/// Refresh Token リポジトリのエラー型
#[derive(Debug, thiserror::Error)]
pub enum RefreshTokenRepositoryError {
    #[error("Token not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// Refresh Token リポジトリのトレイト
#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    /// Refresh Token を保存
    async fn save(&self, token: &RefreshToken) -> Result<(), RefreshTokenRepositoryError>;

    /// トークンハッシュで検索
    async fn find_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<RefreshToken>, RefreshTokenRepositoryError>;

    /// トークンIDで削除（logout）
    async fn delete_by_id(&self, token_id: &Uuid) -> Result<(), RefreshTokenRepositoryError>;

    /// ユーザーIDで全削除（全デバイスからlogout）
    async fn delete_all_by_user_id(&self, user_id: &Uuid)
        -> Result<(), RefreshTokenRepositoryError>;

    /// トークンを無効化
    async fn revoke(&self, token_id: &Uuid) -> Result<(), RefreshTokenRepositoryError>;

    /// 期限切れトークンを削除（定期クリーンアップ用）
    async fn delete_expired(&self) -> Result<u64, RefreshTokenRepositoryError>;
}
