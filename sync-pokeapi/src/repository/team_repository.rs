use crate::domain::{Team, TeamPokemon};
use crate::repository::pokemon_repository::RepositoryError;
use async_trait::async_trait;

/// Team リポジトリのトレイト
#[async_trait]
pub trait TeamRepository: Send + Sync {
    /// チームを作成（IDを返す）
    async fn create_team(&self, team: &Team) -> Result<i32, RepositoryError>;

    /// チームIDでチームを取得
    async fn find_team_by_id(&self, team_id: i32) -> Result<Option<Team>, RepositoryError>;

    /// ユーザーIDで全チームを取得
    async fn find_teams_by_user_id(&self, user_id: &str) -> Result<Vec<Team>, RepositoryError>;

    /// チームを更新
    async fn update_team(&self, team: &Team) -> Result<(), RepositoryError>;

    /// チームを削除（関連するポケモンの関連も削除）
    async fn delete_team(&self, team_id: i32) -> Result<(), RepositoryError>;

    /// チームにポケモンを追加（中間テーブルに登録）
    async fn add_pokemon_to_team(&self, relation: &TeamPokemon) -> Result<(), RepositoryError>;

    /// チーム内のポケモンID一覧を取得
    async fn find_pokemon_ids_by_team_id(&self, team_id: i32)
        -> Result<Vec<i32>, RepositoryError>;

    /// チームからポケモンを削除（中間テーブルから削除）
    async fn remove_pokemon_from_team(
        &self,
        team_id: i32,
        pokemon_id: i32,
    ) -> Result<(), RepositoryError>;

    /// スロット位置を更新
    async fn update_slot(&self, team_id: i32, pokemon_id: i32, slot: i32)
        -> Result<(), RepositoryError>;
}
