use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::domain::entity::refresh_token::{ACCESS_TOKEN_EXPIRY_MINUTES, REFRESH_TOKEN_EXPIRY_DAYS};

/// JWT Claims（ペイロード）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject（user_id）
    pub sub: String,
    /// Expiration time（Unix timestamp）
    pub exp: i64,
    /// Issued at（Unix timestamp）
    pub iat: i64,
    /// Token type（"access" or "refresh"）
    pub token_type: String,
}

impl Claims {
    /// Access Token 用の Claims を生成
    #[must_use]
    pub fn new_access_token(user_id: &Uuid) -> Self {
        let now = Utc::now();
        let exp = now + Duration::minutes(ACCESS_TOKEN_EXPIRY_MINUTES);
        Self {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: "access".to_string(),
        }
    }

    /// Refresh Token 用の Claims を生成
    #[must_use]
    pub fn new_refresh_token(user_id: &Uuid) -> Self {
        let now = Utc::now();
        let exp = now + Duration::days(REFRESH_TOKEN_EXPIRY_DAYS);
        Self {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: "refresh".to_string(),
        }
    }

    /// トークンが期限切れかどうか
    #[must_use]
    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }

    /// Access Token かどうか
    #[must_use]
    pub fn is_access_token(&self) -> bool {
        self.token_type == "access"
    }

    /// Refresh Token かどうか
    #[must_use]
    pub fn is_refresh_token(&self) -> bool {
        self.token_type == "refresh"
    }
}

/// トークンペア（Access Token + Refresh Token）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

impl TokenPair {
    #[must_use]
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: ACCESS_TOKEN_EXPIRY_MINUTES * 60,
        }
    }
}

/// JWT エラー
#[derive(Debug, thiserror::Error)]
pub enum JwtError {
    #[error("Token generation failed: {0}")]
    GenerationFailed(String),
    #[error("Token validation failed: {0}")]
    ValidationFailed(String),
    #[error("Token expired")]
    Expired,
    #[error("Invalid token type")]
    InvalidTokenType,
}

/// JWT サービス
pub struct JwtService {
    secret: String,
}

impl JwtService {
    #[must_use]
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    /// Access Token を生成
    pub fn generate_access_token(&self, user_id: &Uuid) -> Result<String, JwtError> {
        let claims = Claims::new_access_token(user_id);
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| JwtError::GenerationFailed(e.to_string()))
    }

    /// Refresh Token を生成
    pub fn generate_refresh_token(&self, user_id: &Uuid) -> Result<String, JwtError> {
        let claims = Claims::new_refresh_token(user_id);
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| JwtError::GenerationFailed(e.to_string()))
    }

    /// Token Pair を生成
    pub fn generate_token_pair(&self, user_id: &Uuid) -> Result<TokenPair, JwtError> {
        let access_token = self.generate_access_token(user_id)?;
        let refresh_token = self.generate_refresh_token(user_id)?;
        Ok(TokenPair::new(access_token, refresh_token))
    }

    /// トークンを検証して Claims を返す
    pub fn verify_token(&self, token: &str) -> Result<Claims, JwtError> {
        let mut validation = Validation::default();
        validation.validate_exp = true;

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| {
            if e.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                JwtError::Expired
            } else {
                JwtError::ValidationFailed(e.to_string())
            }
        })
    }

    /// Access Token を検証
    pub fn verify_access_token(&self, token: &str) -> Result<Claims, JwtError> {
        let claims = self.verify_token(token)?;
        if claims.is_access_token() {
            Ok(claims)
        } else {
            Err(JwtError::InvalidTokenType)
        }
    }

    /// Refresh Token を検証
    pub fn verify_refresh_token(&self, token: &str) -> Result<Claims, JwtError> {
        let claims = self.verify_token(token)?;
        if claims.is_refresh_token() {
            Ok(claims)
        } else {
            Err(JwtError::InvalidTokenType)
        }
    }

    /// Refresh Token のハッシュを計算（DB保存用）
    #[must_use]
    pub fn hash_refresh_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_access_token() {
        let service = JwtService::new("test_secret".to_string());
        let user_id = Uuid::new_v4();

        let token = service.generate_access_token(&user_id).unwrap();
        let claims = service.verify_access_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert!(claims.is_access_token());
    }

    #[test]
    fn test_generate_and_verify_refresh_token() {
        let service = JwtService::new("test_secret".to_string());
        let user_id = Uuid::new_v4();

        let token = service.generate_refresh_token(&user_id).unwrap();
        let claims = service.verify_refresh_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert!(claims.is_refresh_token());
    }

    #[test]
    fn test_token_pair_generation() {
        let service = JwtService::new("test_secret".to_string());
        let user_id = Uuid::new_v4();

        let pair = service.generate_token_pair(&user_id).unwrap();

        assert!(!pair.access_token.is_empty());
        assert!(!pair.refresh_token.is_empty());
        assert_eq!(pair.token_type, "Bearer");
    }

    #[test]
    fn test_hash_refresh_token() {
        let token = "some_refresh_token";
        let hash = JwtService::hash_refresh_token(token);

        assert_eq!(hash.len(), 64); // SHA256 hex = 64 characters
    }
}
