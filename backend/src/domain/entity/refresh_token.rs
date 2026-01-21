use chrono::{Duration, NaiveDateTime, Utc};
use uuid::Uuid;

/// Access Token の有効期限（分）
pub const ACCESS_TOKEN_EXPIRY_MINUTES: i64 = 15;
/// Refresh Token の有効期限（日）
pub const REFRESH_TOKEN_EXPIRY_DAYS: i64 = 30;

/// Refresh Token エンティティ（DB保存用）
#[derive(Debug, Clone)]
pub struct RefreshToken {
    token_id: Uuid,
    user_id: Uuid,
    token_hash: String,
    expires_at: NaiveDateTime,
    created_at: NaiveDateTime,
    revoked: bool,
}

impl RefreshToken {
    /// 新しい Refresh Token を作成
    #[must_use]
    pub fn new(user_id: Uuid, token_hash: String) -> Self {
        let now = Utc::now().naive_utc();
        let expires_at = now + Duration::days(REFRESH_TOKEN_EXPIRY_DAYS);
        Self {
            token_id: Uuid::new_v4(),
            user_id,
            token_hash,
            expires_at,
            created_at: now,
            revoked: false,
        }
    }

    /// DBから復元する際に使用
    #[must_use]
    pub fn from_repository(
        token_id: Uuid,
        user_id: Uuid,
        token_hash: String,
        expires_at: NaiveDateTime,
        created_at: NaiveDateTime,
        revoked: bool,
    ) -> Self {
        Self {
            token_id,
            user_id,
            token_hash,
            expires_at,
            created_at,
            revoked,
        }
    }

    /// トークンが有効かどうか
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.revoked && Utc::now().naive_utc() < self.expires_at
    }

    /// トークンを無効化
    pub fn revoke(&mut self) {
        self.revoked = true;
    }

    // Getters
    #[must_use]
    pub fn token_id(&self) -> &Uuid {
        &self.token_id
    }

    #[must_use]
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    #[must_use]
    pub fn token_hash(&self) -> &str {
        &self.token_hash
    }

    #[must_use]
    pub fn expires_at(&self) -> &NaiveDateTime {
        &self.expires_at
    }

    #[must_use]
    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    #[must_use]
    pub fn is_revoked(&self) -> bool {
        self.revoked
    }
}
