use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entity::user::User;
use crate::domain::valueobject::email::Email;
use crate::domain::valueobject::username::Username;
use crate::repository::user_repository::{UserRepository, UserRepositoryError};

/// PostgreSQLユーザーリポジトリ
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
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
    pub async fn migrate(&self) -> Result<(), UserRepositoryError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                user_id UUID PRIMARY KEY,
                username VARCHAR(20) NOT NULL UNIQUE,
                email VARCHAR(254) NOT NULL UNIQUE,
                password_hash VARCHAR(60) NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO users (user_id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (user_id) DO UPDATE
            SET username = $2, email = $3, password_hash = $4, updated_at = NOW()
            "#,
        )
        .bind(user.user_id())
        .bind(user.username().as_str())
        .bind(user.email().as_str())
        .bind(user.password_hash())
        .execute(&self.pool)
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, UserRepositoryError> {
        let row = sqlx::query_as::<_, (Uuid, String, String, String)>(
            r#"
            SELECT user_id, username, email, password_hash
            FROM users
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some((user_id, username, email, password_hash)) => {
                let username = Username::new(&username).map_err(|e| {
                    UserRepositoryError::DatabaseError(format!("Invalid username: {e}"))
                })?;
                let email = Email::new(&email).map_err(|e| {
                    UserRepositoryError::DatabaseError(format!("Invalid email: {e}"))
                })?;
                let user = User::from_repository(user_id, username, email, &password_hash)
                    .map_err(|e| {
                        UserRepositoryError::DatabaseError(format!("Invalid password hash: {e}"))
                    })?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError> {
        let row = sqlx::query_as::<_, (Uuid, String, String, String)>(
            r#"
            SELECT user_id, username, email, password_hash
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some((user_id, username, email, password_hash)) => {
                let username = Username::new(&username).map_err(|e| {
                    UserRepositoryError::DatabaseError(format!("Invalid username: {e}"))
                })?;
                let email = Email::new(&email).map_err(|e| {
                    UserRepositoryError::DatabaseError(format!("Invalid email: {e}"))
                })?;
                let user = User::from_repository(user_id, username, email, &password_hash)
                    .map_err(|e| {
                        UserRepositoryError::DatabaseError(format!("Invalid password hash: {e}"))
                    })?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn find_by_username(
        &self,
        username: &Username,
    ) -> Result<Option<User>, UserRepositoryError> {
        let row = sqlx::query_as::<_, (Uuid, String, String, String)>(
            r#"
            SELECT user_id, username, email, password_hash
            FROM users
            WHERE username = $1
            "#,
        )
        .bind(username.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some((user_id, username, email, password_hash)) => {
                let username = Username::new(&username).map_err(|e| {
                    UserRepositoryError::DatabaseError(format!("Invalid username: {e}"))
                })?;
                let email = Email::new(&email).map_err(|e| {
                    UserRepositoryError::DatabaseError(format!("Invalid email: {e}"))
                })?;
                let user = User::from_repository(user_id, username, email, &password_hash)
                    .map_err(|e| {
                        UserRepositoryError::DatabaseError(format!("Invalid password hash: {e}"))
                    })?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn exists_by_email(&self, email: &Email) -> Result<bool, UserRepositoryError> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)
            "#,
        )
        .bind(email.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(exists)
    }

    async fn exists_by_username(&self, username: &Username) -> Result<bool, UserRepositoryError> {
        let exists = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)
            "#,
        )
        .bind(username.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| UserRepositoryError::DatabaseError(e.to_string()))?;

        Ok(exists)
    }
}
