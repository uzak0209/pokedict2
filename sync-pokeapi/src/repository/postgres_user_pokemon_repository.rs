use crate::domain::UserPokemon;
use crate::repository::pokemon_repository::RepositoryError;
use crate::repository::user_pokemon_repository::UserPokemonRepository;
use async_trait::async_trait;
use sqlx::{PgPool, Row};

// SQL queries
const CREATE_SQL: &str = include_str!("../sql/user_pokemon/create.sql");
const FIND_BY_ID_SQL: &str = include_str!("../sql/user_pokemon/find_by_id.sql");
const FIND_BY_USER_ID_SQL: &str = include_str!("../sql/user_pokemon/find_by_user_id.sql");
const UPDATE_SQL: &str = include_str!("../sql/user_pokemon/update.sql");
const DELETE_SQL: &str = include_str!("../sql/user_pokemon/delete.sql");

/// PostgreSQL UserPokemon リポジトリ実装
pub struct PostgresUserPokemonRepository {
    pool: PgPool,
}

impl PostgresUserPokemonRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserPokemonRepository for PostgresUserPokemonRepository {
    async fn create_pokemon(&self, pokemon: &UserPokemon) -> Result<i32, RepositoryError> {
        let result = sqlx::query_as::<_, (i32,)>(CREATE_SQL)
        .bind(&pokemon.user_id)
        .bind(pokemon.form_id)
        .bind(&pokemon.nickname)
        .bind(pokemon.level)
        .bind(&pokemon.nature)
        .bind(&pokemon.ability)
        .bind(&pokemon.item)
        .bind(&pokemon.tera_type)
        .bind(&pokemon.move1)
        .bind(&pokemon.move2)
        .bind(&pokemon.move3)
        .bind(&pokemon.move4)
        .bind(pokemon.ev_hp)
        .bind(pokemon.ev_attack)
        .bind(pokemon.ev_defense)
        .bind(pokemon.ev_sp_attack)
        .bind(pokemon.ev_sp_defense)
        .bind(pokemon.ev_speed)
        .bind(pokemon.iv_hp)
        .bind(pokemon.iv_attack)
        .bind(pokemon.iv_defense)
        .bind(pokemon.iv_sp_attack)
        .bind(pokemon.iv_sp_defense)
        .bind(pokemon.iv_speed)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.0)
    }

    async fn find_pokemon_by_id(
        &self,
        pokemon_id: i32,
    ) -> Result<Option<UserPokemon>, RepositoryError> {
        let row = sqlx::query(FIND_BY_ID_SQL)
        .bind(pokemon_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| UserPokemon {
            pokemon_id: r.get("pokemon_id"),
            user_id: r.get("user_id"),
            form_id: r.get("form_id"),
            nickname: r.get("nickname"),
            level: r.get("level"),
            nature: r.get("nature"),
            ability: r.get("ability"),
            item: r.get("item"),
            tera_type: r.get("tera_type"),
            move1: r.get("move1"),
            move2: r.get("move2"),
            move3: r.get("move3"),
            move4: r.get("move4"),
            ev_hp: r.get("ev_hp"),
            ev_attack: r.get("ev_attack"),
            ev_defense: r.get("ev_defense"),
            ev_sp_attack: r.get("ev_sp_attack"),
            ev_sp_defense: r.get("ev_sp_defense"),
            ev_speed: r.get("ev_speed"),
            iv_hp: r.get("iv_hp"),
            iv_attack: r.get("iv_attack"),
            iv_defense: r.get("iv_defense"),
            iv_sp_attack: r.get("iv_sp_attack"),
            iv_sp_defense: r.get("iv_sp_defense"),
            iv_speed: r.get("iv_speed"),
            created_at: r.get("created_at"),
        }))
    }

    async fn find_pokemon_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<UserPokemon>, RepositoryError> {
        let rows = sqlx::query(FIND_BY_USER_ID_SQL)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|r| UserPokemon {
                pokemon_id: r.get("pokemon_id"),
                user_id: r.get("user_id"),
                form_id: r.get("form_id"),
                nickname: r.get("nickname"),
                level: r.get("level"),
                nature: r.get("nature"),
                ability: r.get("ability"),
                item: r.get("item"),
                tera_type: r.get("tera_type"),
                move1: r.get("move1"),
                move2: r.get("move2"),
                move3: r.get("move3"),
                move4: r.get("move4"),
                ev_hp: r.get("ev_hp"),
                ev_attack: r.get("ev_attack"),
                ev_defense: r.get("ev_defense"),
                ev_sp_attack: r.get("ev_sp_attack"),
                ev_sp_defense: r.get("ev_sp_defense"),
                ev_speed: r.get("ev_speed"),
                iv_hp: r.get("iv_hp"),
                iv_attack: r.get("iv_attack"),
                iv_defense: r.get("iv_defense"),
                iv_sp_attack: r.get("iv_sp_attack"),
                iv_sp_defense: r.get("iv_sp_defense"),
                iv_speed: r.get("iv_speed"),
                created_at: r.get("created_at"),
            })
            .collect())
    }

    async fn update_pokemon(&self, pokemon: &UserPokemon) -> Result<(), RepositoryError> {
        sqlx::query(UPDATE_SQL)
        .bind(pokemon.pokemon_id)
        .bind(pokemon.form_id)
        .bind(&pokemon.nickname)
        .bind(pokemon.level)
        .bind(&pokemon.nature)
        .bind(&pokemon.ability)
        .bind(&pokemon.item)
        .bind(&pokemon.tera_type)
        .bind(&pokemon.move1)
        .bind(&pokemon.move2)
        .bind(&pokemon.move3)
        .bind(&pokemon.move4)
        .bind(pokemon.ev_hp)
        .bind(pokemon.ev_attack)
        .bind(pokemon.ev_defense)
        .bind(pokemon.ev_sp_attack)
        .bind(pokemon.ev_sp_defense)
        .bind(pokemon.ev_speed)
        .bind(pokemon.iv_hp)
        .bind(pokemon.iv_attack)
        .bind(pokemon.iv_defense)
        .bind(pokemon.iv_sp_attack)
        .bind(pokemon.iv_sp_defense)
        .bind(pokemon.iv_speed)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_pokemon(&self, pokemon_id: i32) -> Result<(), RepositoryError> {
        sqlx::query(DELETE_SQL)
            .bind(pokemon_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
