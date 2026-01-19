use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entity::team::Team;

/// チームリポジトリのエラー型
#[derive(Debug, thiserror::Error)]
pub enum TeamRepositoryError {
    #[error("Team not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// チームリポジトリのトレイト
#[async_trait]
pub trait TeamRepository: Send + Sync {
    /// チームを保存
    async fn save(&self, team: &Team) -> Result<(), TeamRepositoryError>;

    /// チームIDでチームを取得
    async fn find_by_id(&self, team_id: &Uuid) -> Result<Option<Team>, TeamRepositoryError>;

    /// ユーザーIDで所有するチームリストを取得
    async fn find_by_owner_id(&self, owner_id: &Uuid) -> Result<Vec<Team>, TeamRepositoryError>;

    /// チームを削除
    async fn delete(&self, team_id: &Uuid) -> Result<(), TeamRepositoryError>;

    /// チームを更新
    async fn update(&self, team: &Team) -> Result<(), TeamRepositoryError>;
}
