use crate::domain::User;
use crate::repository::pokemon_repository::RepositoryError;
use crate::repository::user_repository::UserRepository;
use async_trait::async_trait;
use sqlx::PgPool;

/// PostgreSQL User リポジトリ実装
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, user: &User) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO users (user_id, username, email, created_at)
            VALUES ($1, $2, $3, NOW())
            "#,
        )
        .bind(&user.user_id)
        .bind(&user.username)
        .bind(&user.email)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_user_by_id(&self, user_id: &str) -> Result<Option<User>, RepositoryError> {
        let result = sqlx::query_as::<_, (String, String, Option<String>, chrono::NaiveDateTime)>(
            "SELECT user_id, username, email, created_at FROM users WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.map(|(user_id, username, email, created_at)| User {
            user_id,
            username,
            email,
            created_at: Some(created_at),
        }))
    }

    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError> {
        let result = sqlx::query_as::<_, (String, String, Option<String>, chrono::NaiveDateTime)>(
            "SELECT user_id, username, email, created_at FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.map(|(user_id, username, email, created_at)| User {
            user_id,
            username,
            email,
            created_at: Some(created_at),
        }))
    }

    async fn update_user(&self, user: &User) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            UPDATE users
            SET username = $2, email = $3
            WHERE user_id = $1
            "#,
        )
        .bind(&user.user_id)
        .bind(&user.username)
        .bind(&user.email)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM users WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
