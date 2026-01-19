use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use crate::domain::entity::team::Team;
use crate::repository::team_repository::{TeamRepository, TeamRepositoryError};

/// インメモリチームリポジトリ（開発・テスト用）
pub struct MockTeamRepository {
    teams: Mutex<HashMap<Uuid, Team>>,
}

impl MockTeamRepository {
    /// 新しいモックリポジトリを作成
    #[must_use]
    pub fn new() -> Self {
        Self {
            teams: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for MockTeamRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TeamRepository for MockTeamRepository {
    async fn save(&self, team: &Team) -> Result<(), TeamRepositoryError> {
        let mut teams = self
            .teams
            .lock()
            .map_err(|e| TeamRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
        teams.insert(*team.team_id(), team.clone());
        Ok(())
    }

    async fn find_by_id(&self, team_id: &Uuid) -> Result<Option<Team>, TeamRepositoryError> {
        let teams = self
            .teams
            .lock()
            .map_err(|e| TeamRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
        Ok(teams.get(team_id).cloned())
    }

    async fn find_by_owner_id(
        &self,
        owner_id: &Uuid,
    ) -> Result<Vec<Team>, TeamRepositoryError> {
        let teams = self
            .teams
            .lock()
            .map_err(|e| TeamRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
        Ok(teams
            .values()
            .filter(|t| t.owner_id() == owner_id)
            .cloned()
            .collect())
    }

    async fn delete(&self, team_id: &Uuid) -> Result<(), TeamRepositoryError> {
        let mut teams = self
            .teams
            .lock()
            .map_err(|e| TeamRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
        teams
            .remove(team_id)
            .ok_or(TeamRepositoryError::NotFound)?;
        Ok(())
    }

    async fn update(&self, team: &Team) -> Result<(), TeamRepositoryError> {
        let mut teams = self
            .teams
            .lock()
            .map_err(|e| TeamRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
        teams
            .get_mut(team.team_id())
            .ok_or(TeamRepositoryError::NotFound)?;
        teams.insert(*team.team_id(), team.clone());
        Ok(())
    }
}
