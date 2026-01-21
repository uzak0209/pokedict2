use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entity::refresh_token::RefreshToken;
use crate::repository::refresh_token_repository::{
    RefreshTokenRepository, RefreshTokenRepositoryError,
};

/// PostgreSQL Refresh Token リポジトリ
pub struct PostgresRefreshTokenRepository {
    pool: PgPool,
}

impl PostgresRefreshTokenRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// データベースマイグレーションを実行
    ///
    /// # Errors
    ///
    /// - マイグレーション実行に失敗した場合
    pub async fn migrate(&self) -> Result<(), RefreshTokenRepositoryError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS refresh_tokens (
                token_id UUID PRIMARY KEY,
                user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
                token_hash VARCHAR(64) NOT NULL,
                expires_at TIMESTAMPTZ NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                revoked BOOLEAN NOT NULL DEFAULT FALSE
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RefreshTokenRepositoryError::DatabaseError(e.to_string()))?;

        // インデックス作成
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user_id ON refresh_tokens(user_id)",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RefreshTokenRepositoryError::DatabaseError(e.to_string()))?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_refresh_tokens_hash ON refresh_tokens(token_hash)",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RefreshTokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn save(&self, token: &RefreshToken) -> Result<(), RefreshTokenRepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO refresh_tokens (token_id, user_id, token_hash, expires_at, created_at, revoked)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(token.token_id())
        .bind(token.user_id())
        .bind(token.token_hash())
        .bind(token.expires_at())
        .bind(token.created_at())
        .bind(token.is_revoked())
        .execute(&self.pool)
        .await
        .map_err(|e| RefreshTokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<RefreshToken>, RefreshTokenRepositoryError> {
        let row: Option<(Uuid, Uuid, String, chrono::NaiveDateTime, chrono::NaiveDateTime, bool)> =
            sqlx::query_as(
                r#"
                SELECT token_id, user_id, token_hash, expires_at, created_at, revoked
                FROM refresh_tokens
                WHERE token_hash = $1
                "#,
            )
            .bind(token_hash)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RefreshTokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|(token_id, user_id, token_hash, expires_at, created_at, revoked)| {
            RefreshToken::from_repository(token_id, user_id, token_hash, expires_at, created_at, revoked)
        }))
    }

    async fn delete_by_id(&self, token_id: &Uuid) -> Result<(), RefreshTokenRepositoryError> {
        sqlx::query("DELETE FROM refresh_tokens WHERE token_id = $1")
            .bind(token_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RefreshTokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_all_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> Result<(), RefreshTokenRepositoryError> {
        sqlx::query("DELETE FROM refresh_tokens WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RefreshTokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn revoke(&self, token_id: &Uuid) -> Result<(), RefreshTokenRepositoryError> {
        sqlx::query("UPDATE refresh_tokens SET revoked = true WHERE token_id = $1")
            .bind(token_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RefreshTokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64, RefreshTokenRepositoryError> {
        let result = sqlx::query("DELETE FROM refresh_tokens WHERE expires_at < NOW()")
            .execute(&self.pool)
            .await
            .map_err(|e| RefreshTokenRepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected())
    }
}
