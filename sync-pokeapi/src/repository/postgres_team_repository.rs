use crate::domain::{Team, TeamPokemon};
use crate::repository::pokemon_repository::RepositoryError;
use crate::repository::team_repository::TeamRepository;
use async_trait::async_trait;
use sqlx::PgPool;

// SQL queries
const CREATE_SQL: &str = include_str!("../sql/team/create.sql");
const FIND_BY_ID_SQL: &str = include_str!("../sql/team/find_by_id.sql");
const FIND_BY_USER_ID_SQL: &str = include_str!("../sql/team/find_by_user_id.sql");
const UPDATE_SQL: &str = include_str!("../sql/team/update.sql");
const DELETE_SQL: &str = include_str!("../sql/team/delete.sql");
const ADD_POKEMON_SQL: &str = include_str!("../sql/team/add_pokemon.sql");
const FIND_POKEMON_IDS_SQL: &str = include_str!("../sql/team/find_pokemon_ids.sql");
const REMOVE_POKEMON_SQL: &str = include_str!("../sql/team/remove_pokemon.sql");
const UPDATE_SLOT_SQL: &str = include_str!("../sql/team/update_slot.sql");

/// PostgreSQL Team リポジトリ実装
pub struct PostgresTeamRepository {
    pool: PgPool,
}

impl PostgresTeamRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TeamRepository for PostgresTeamRepository {
    async fn create_team(&self, team: &Team) -> Result<i32, RepositoryError> {
        let result = sqlx::query_as::<_, (i32,)>(CREATE_SQL)
        .bind(&team.user_id)
        .bind(&team.team_name)
        .bind(&team.description)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.0)
    }

    async fn find_team_by_id(&self, team_id: i32) -> Result<Option<Team>, RepositoryError> {
        let result = sqlx::query_as::<
            _,
            (
                i32,
                String,
                String,
                Option<String>,
                chrono::NaiveDateTime,
                chrono::NaiveDateTime,
            ),
        >(FIND_BY_ID_SQL)
        .bind(team_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.map(
            |(team_id, user_id, team_name, description, created_at, updated_at)| Team {
                team_id,
                user_id,
                team_name,
                description,
                created_at: Some(created_at),
                updated_at: Some(updated_at),
            },
        ))
    }

    async fn find_teams_by_user_id(&self, user_id: &str) -> Result<Vec<Team>, RepositoryError> {
        let results = sqlx::query_as::<
            _,
            (
                i32,
                String,
                String,
                Option<String>,
                chrono::NaiveDateTime,
                chrono::NaiveDateTime,
            ),
        >(FIND_BY_USER_ID_SQL)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(results
            .into_iter()
            .map(
                |(team_id, user_id, team_name, description, created_at, updated_at)| Team {
                    team_id,
                    user_id,
                    team_name,
                    description,
                    created_at: Some(created_at),
                    updated_at: Some(updated_at),
                },
            )
            .collect())
    }

    async fn update_team(&self, team: &Team) -> Result<(), RepositoryError> {
        sqlx::query(UPDATE_SQL)
        .bind(team.team_id)
        .bind(&team.team_name)
        .bind(&team.description)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_team(&self, team_id: i32) -> Result<(), RepositoryError> {
        sqlx::query(DELETE_SQL)
            .bind(team_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn add_pokemon_to_team(&self, relation: &TeamPokemon) -> Result<(), RepositoryError> {
        sqlx::query(ADD_POKEMON_SQL)
        .bind(relation.team_id)
        .bind(relation.pokemon_id)
        .bind(relation.slot)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_pokemon_ids_by_team_id(
        &self,
        team_id: i32,
    ) -> Result<Vec<i32>, RepositoryError> {
        let rows = sqlx::query_as::<_, (i32,)>(FIND_POKEMON_IDS_SQL)
        .bind(team_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|(id,)| id).collect())
    }

    async fn remove_pokemon_from_team(
        &self,
        team_id: i32,
        pokemon_id: i32,
    ) -> Result<(), RepositoryError> {
        sqlx::query(REMOVE_POKEMON_SQL)
            .bind(team_id)
            .bind(pokemon_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn update_slot(
        &self,
        team_id: i32,
        pokemon_id: i32,
        slot: i32,
    ) -> Result<(), RepositoryError> {
        sqlx::query(UPDATE_SLOT_SQL)
        .bind(team_id)
        .bind(pokemon_id)
        .bind(slot)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
