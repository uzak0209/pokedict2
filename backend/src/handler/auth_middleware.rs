use actix_web::{dev::ServiceRequest, Error, HttpMessage, HttpResponse, web};
use actix_web::error::ErrorUnauthorized;
use std::sync::Arc;
use uuid::Uuid;

use crate::repository::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
use crate::repository::postgres_user_repository::PostgresUserRepository;
use crate::usecase::auth_service::AuthService;

/// 認証済みユーザー情報（リクエストエクステンションとして使用）
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl AuthenticatedUser {
    #[must_use]
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

/// Authorization ヘッダーからBearerトークンを抽出
fn extract_bearer_token(req: &ServiceRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].to_string())
            } else {
                None
            }
        })
}

/// トークンを検証してユーザーIDを取得
pub fn validate_token(
    auth_service: &AuthService<PostgresUserRepository, PostgresRefreshTokenRepository>,
    token: &str,
) -> Result<Uuid, String> {
    auth_service
        .validate_access_token(token)
        .map_err(|e| e.to_string())
}

/// リクエストから認証済みユーザーを取得するヘルパー
/// 
/// ハンドラー内で使用：
/// ```ignore
/// let user = get_authenticated_user(&req)?;
/// let user_id = user.user_id;
/// ```
pub fn get_authenticated_user_from_request(
    req: &actix_web::HttpRequest,
) -> Result<AuthenticatedUser, HttpResponse> {
    req.extensions()
        .get::<AuthenticatedUser>()
        .cloned()
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required",
                "error_code": "UNAUTHORIZED"
            }))
        })
}

/// 認証が必要なエンドポイント用のガード関数
/// 
/// ハンドラーの最初で呼び出し、認証済みユーザーIDを取得する
pub async fn require_auth(
    req: &actix_web::HttpRequest,
    auth_service: &web::Data<Arc<AuthService<PostgresUserRepository, PostgresRefreshTokenRepository>>>,
) -> Result<Uuid, HttpResponse> {
    // Authorizationヘッダーからトークンを取得
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Missing or invalid Authorization header",
                "error_code": "MISSING_AUTH"
            }))
        })?;

    // トークンを検証
    auth_service.validate_access_token(&token).map_err(|e| {
        HttpResponse::Unauthorized().json(serde_json::json!({
            "error": format!("Invalid token: {}", e),
            "error_code": "INVALID_TOKEN"
        }))
    })
}
