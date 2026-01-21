use crate::domain::User;
use crate::repository::pokemon_repository::RepositoryError;
use async_trait::async_trait;

/// User リポジトリのトレイト
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// ユーザーを作成
    async fn create_user(&self, user: &User) -> Result<(), RepositoryError>;

    /// ユーザーIDでユーザーを取得
    async fn find_user_by_id(&self, user_id: &str) -> Result<Option<User>, RepositoryError>;

    /// ユーザー名でユーザーを取得
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError>;

    /// ユーザーを更新
    async fn update_user(&self, user: &User) -> Result<(), RepositoryError>;

    /// ユーザーを削除
    async fn delete_user(&self, user_id: &str) -> Result<(), RepositoryError>;
}
