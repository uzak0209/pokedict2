use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entity::user::User;
use crate::domain::valueobject::email::Email;
use crate::domain::valueobject::username::Username;

/// ユーザーリポジトリのエラー型
#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// ユーザーリポジトリのトレイト
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// ユーザーを保存
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;

    /// IDでユーザーを取得
    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, UserRepositoryError>;

    /// メールアドレスでユーザーを取得
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError>;

    /// ユーザー名でユーザーを取得
    async fn find_by_username(
        &self,
        username: &Username,
    ) -> Result<Option<User>, UserRepositoryError>;

    /// メールアドレスが既に登録されているか確認
    async fn exists_by_email(&self, email: &Email) -> Result<bool, UserRepositoryError>;

    /// ユーザー名が既に登録されているか確認
    async fn exists_by_username(&self, username: &Username) -> Result<bool, UserRepositoryError>;
}
