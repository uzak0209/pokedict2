use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entity::team::Team;
use crate::domain::valueobject::teamname::TeamName;
use crate::repository::team_repository::{TeamRepository, TeamRepositoryError};

/// PostgreSQLチームリポジトリ
pub struct PostgresTeamRepository {
    pool: PgPool,
}

impl PostgresTeamRepository {
    /// 新しいPostgreSQLリポジトリを作成
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// データベースマイグレーションを実行
    ///
    /// # Errors
    ///
    /// - マイグレーション実行に失敗した場合
    pub async fn migrate(&self) -> Result<(), TeamRepositoryError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS teams (
                team_id UUID PRIMARY KEY,
                owner_id UUID NOT NULL,
                team_name VARCHAR(30) NOT NULL,
                pokemon_data JSONB NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                FOREIGN KEY (owner_id) REFERENCES users(user_id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| TeamRepositoryError::DatabaseError(e.to_string()))?;

        // インデックスを作成
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_teams_owner_id ON teams(owner_id)
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| TeamRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl TeamRepository for PostgresTeamRepository {
    async fn save(&self, team: &Team) -> Result<(), TeamRepositoryError> {
        // PokemonFormをJSONに変換
        let pokemon_data = serde_json::to_value(team.pokemon())
            .map_err(|e| TeamRepositoryError::DatabaseError(format!("Failed to serialize pokemon: {e}")))?;

        sqlx::query(
            r#"
            INSERT INTO teams (team_id, owner_id, team_name, pokemon_data)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (team_id) DO UPDATE
            SET team_name = $3, pokemon_data = $4, updated_at = NOW()
            "#,
        )
        .bind(team.team_id())
        .bind(team.owner_id())
        .bind(team.team_name().as_str())
        .bind(pokemon_data)
        .execute(&self.pool)
        .await
        .map_err(|e| TeamRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, team_id: &Uuid) -> Result<Option<Team>, TeamRepositoryError> {
        let row = sqlx::query_as::<_, (Uuid, Uuid, String, serde_json::Value)>(
            r#"
            SELECT team_id, owner_id, team_name, pokemon_data
            FROM teams
            WHERE team_id = $1
            "#,
        )
        .bind(team_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| TeamRepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some((team_id, owner_id, team_name, pokemon_data)) => {
                let team_name = TeamName::new(&team_name).map_err(|e| {
                    TeamRepositoryError::DatabaseError(format!("Invalid team name: {e}"))
                })?;

                // JSONからVecに変換
                let pokemon_vec: Vec<Option<crate::domain::entity::pokemon_form::PokemonForm>> = serde_json::from_value(pokemon_data).map_err(|e| {
                    TeamRepositoryError::DatabaseError(format!("Failed to deserialize pokemon: {e}"))
                })?;

                let team = Team::from_repository(team_id, owner_id, team_name, pokemon_vec)
                    .map_err(|e| TeamRepositoryError::DatabaseError(e.to_string()))?;

                Ok(Some(team))
            }
            None => Ok(None),
        }
    }

    async fn find_by_owner_id(
        &self,
        owner_id: &Uuid,
    ) -> Result<Vec<Team>, TeamRepositoryError> {
        let rows = sqlx::query_as::<_, (Uuid, Uuid, String, serde_json::Value)>(
            r#"
            SELECT team_id, owner_id, team_name, pokemon_data
            FROM teams
            WHERE owner_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| TeamRepositoryError::DatabaseError(e.to_string()))?;

        let teams: Result<Vec<Team>, TeamRepositoryError> = rows
            .into_iter()
            .map(|(team_id, owner_id, team_name, pokemon_data)| {
                let team_name = TeamName::new(&team_name).map_err(|e| {
                    TeamRepositoryError::DatabaseError(format!("Invalid team name: {e}"))
                })?;

                let pokemon_vec: Vec<Option<crate::domain::entity::pokemon_form::PokemonForm>> = serde_json::from_value(pokemon_data).map_err(|e| {
                    TeamRepositoryError::DatabaseError(format!(
                        "Failed to deserialize pokemon: {e}"
                    ))
                })?;

                Team::from_repository(team_id, owner_id, team_name, pokemon_vec)
                    .map_err(|e| TeamRepositoryError::DatabaseError(e.to_string()))
            })
            .collect();

        teams
    }

    async fn delete(&self, team_id: &Uuid) -> Result<(), TeamRepositoryError> {
        let result = sqlx::query(
            r#"
            DELETE FROM teams WHERE team_id = $1
            "#,
        )
        .bind(team_id)
        .execute(&self.pool)
        .await
        .map_err(|e| TeamRepositoryError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(TeamRepositoryError::NotFound);
        }

        Ok(())
    }

    async fn update(&self, team: &Team) -> Result<(), TeamRepositoryError> {
        let pokemon_data = serde_json::to_value(team.pokemon())
            .map_err(|e| TeamRepositoryError::DatabaseError(format!("Failed to serialize pokemon: {e}")))?;

        let result = sqlx::query(
            r#"
            UPDATE teams
            SET team_name = $1, pokemon_data = $2, updated_at = NOW()
            WHERE team_id = $3
            "#,
        )
        .bind(team.team_name().as_str())
        .bind(pokemon_data)
        .bind(team.team_id())
        .execute(&self.pool)
        .await
        .map_err(|e| TeamRepositoryError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(TeamRepositoryError::NotFound);
        }

        Ok(())
    }
}
