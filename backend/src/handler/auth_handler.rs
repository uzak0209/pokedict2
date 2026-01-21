use actix_web::{cookie::Cookie, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use typeshare::typeshare;

use crate::domain::valueobject::jwt::TokenPair;
use crate::repository::refresh_token_repository::RefreshTokenRepository;
use crate::repository::user_repository::UserRepository;
use crate::usecase::auth_service::{AuthError, AuthService, LoginRequest};
use crate::usecase::user_registration::{
    RegisterUserError, RegisterUserRequest, UserRegistrationUseCase,
};

/// ログインリクエストDTO
#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestDto {
    pub email: String,
    pub password: String,
}

/// トークンレスポンスDTO (refresh_tokenはCookieで返すため除外)
#[typeshare]
#[derive(Debug, Serialize)]
pub struct TokenResponseDto {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub access_token: String,
    pub token_type: String,
    #[typeshare(serialized_as = "number")]
    pub expires_in: i64,
}

/// シンプルトークンレスポンスDTO（refresh用、refresh_tokenはCookieで返す）
#[typeshare]
#[derive(Debug, Serialize)]
pub struct RefreshResponseDto {
    pub access_token: String,
    pub token_type: String,
    #[typeshare(serialized_as = "number")]
    pub expires_in: i64,
}

/// リフレッシュリクエストDTO
#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshRequestDto {
    pub refresh_token: String,
}

/// エラーレスポンスDTO
#[typeshare]
#[derive(Debug, Serialize)]
pub struct ErrorResponseDto {
    pub error: String,
    pub error_code: String,
}

/// ユーザー登録リクエストDTO
#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequestDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// ユーザー登録レスポンスDTO
#[typeshare]
#[derive(Debug, Serialize)]
pub struct RegisterResponseDto {
    pub user_id: String,
    pub username: String,
    pub email: String,
}

/// ログインハンドラー（Token Pair返却版）
///
/// POST /api/auth/login
pub async fn login<U: UserRepository + 'static, R: RefreshTokenRepository + 'static>(
    req: web::Json<LoginRequestDto>,
    auth_service: web::Data<Arc<AuthService<U, R>>>,
) -> impl Responder {
    let request = LoginRequest {
        email: req.email.clone(),
        password: req.password.clone(),
    };

    match auth_service.login(request).await {
        Ok(response) => {
            // refresh_tokenをHTTPOnly Cookieとして設定
            let cookie = Cookie::build("refresh_token", response.token_pair.refresh_token.clone())
                .path("/")
                .http_only(true)
                .secure(false) // 本番環境ではtrueに設定 (HTTPS必須)
                .same_site(actix_web::cookie::SameSite::Lax)
                .max_age(actix_web::cookie::time::Duration::days(30))
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(TokenResponseDto {
                    user_id: response.user_id,
                    username: response.username,
                    email: response.email,
                    access_token: response.token_pair.access_token,
                    token_type: response.token_pair.token_type,
                    expires_in: response.token_pair.expires_in,
                })
        }
        Err(err) => map_auth_error(err),
    }
}

/// トークンリフレッシュハンドラー
///
/// POST /api/auth/refresh
pub async fn refresh<U: UserRepository + 'static, R: RefreshTokenRepository + 'static>(
    http_req: HttpRequest,
    auth_service: web::Data<Arc<AuthService<U, R>>>,
) -> impl Responder {
    // CookieからRefresh Tokenを取得
    let refresh_token = match http_req.cookie("refresh_token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return HttpResponse::Unauthorized().json(ErrorResponseDto {
                error: "Refresh token not found in cookie".to_string(),
                error_code: "MISSING_REFRESH_TOKEN".to_string(),
            });
        }
    };

    match auth_service.refresh(&refresh_token).await {
        Ok(token_pair) => {
            // 新しいrefresh_tokenをHTTPOnly Cookieとして設定
            let cookie = Cookie::build("refresh_token", token_pair.refresh_token.clone())
                .path("/")
                .http_only(true)
                .secure(false) // 本番環境ではtrueに設定 (HTTPS必須)
                .same_site(actix_web::cookie::SameSite::Lax)
                .max_age(actix_web::cookie::time::Duration::days(30))
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(RefreshResponseDto {
                    access_token: token_pair.access_token,
                    token_type: token_pair.token_type,
                    expires_in: token_pair.expires_in,
                })
        }
        Err(err) => map_auth_error(err),
    }
}

/// ログアウトハンドラー
///
/// POST /api/auth/logout
pub async fn logout<U: UserRepository + 'static, R: RefreshTokenRepository + 'static>(
    http_req: HttpRequest,
    auth_service: web::Data<Arc<AuthService<U, R>>>,
) -> impl Responder {
    // CookieからRefresh Tokenを取得
    let refresh_token = match http_req.cookie("refresh_token") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            // refresh_tokenがない場合でもログアウトは成功とする
            let mut removal_cookie = Cookie::named("refresh_token");
            removal_cookie.set_path("/");
            removal_cookie.make_removal();
            return HttpResponse::NoContent().cookie(removal_cookie).finish();
        }
    };

    match auth_service.logout(&refresh_token).await {
        Ok(()) => {
            // Cookieを削除
            let mut removal_cookie = Cookie::named("refresh_token");
            removal_cookie.set_path("/");
            removal_cookie.make_removal();
            HttpResponse::NoContent().cookie(removal_cookie).finish()
        }
        Err(err) => map_auth_error(err),
    }
}

/// 認証エラーをHTTPレスポンスに変換
fn map_auth_error(err: AuthError) -> HttpResponse {
    let (status, code, message): (actix_web::http::StatusCode, &str, String) = match err {
        AuthError::InvalidCredentials => (
            actix_web::http::StatusCode::UNAUTHORIZED,
            "INVALID_CREDENTIALS",
            "Invalid email or password".to_string(),
        ),
        AuthError::InvalidToken => (
            actix_web::http::StatusCode::UNAUTHORIZED,
            "INVALID_TOKEN",
            "Invalid or malformed token".to_string(),
        ),
        AuthError::TokenExpired => (
            actix_web::http::StatusCode::UNAUTHORIZED,
            "TOKEN_EXPIRED",
            "Token has expired".to_string(),
        ),
        AuthError::TokenRevoked => (
            actix_web::http::StatusCode::UNAUTHORIZED,
            "TOKEN_REVOKED",
            "Token has been revoked".to_string(),
        ),
        AuthError::UserNotFound => (
            actix_web::http::StatusCode::NOT_FOUND,
            "USER_NOT_FOUND",
            "User not found".to_string(),
        ),
        AuthError::Repository(msg) => (
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            "REPOSITORY_ERROR",
            msg,
        ),
        AuthError::Jwt(msg) => (
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            "JWT_ERROR",
            msg,
        ),
    };

    HttpResponse::build(status).json(ErrorResponseDto {
        error: message,
        error_code: code.to_string(),
    })
}

/// Authorization ヘッダーから Bearer トークンを抽出
#[must_use]
pub fn extract_bearer_token(req: &HttpRequest) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(String::from)
}

/// ユーザー登録ハンドラー
///
/// POST /api/auth/register
pub async fn register<U: UserRepository + 'static>(
    req: web::Json<RegisterRequestDto>,
    user_repository: web::Data<Arc<U>>,
) -> impl Responder {
    let usecase = UserRegistrationUseCase::new(user_repository.get_ref().clone());

    let request = RegisterUserRequest {
        username: req.username.clone(),
        email: req.email.clone(),
        password: req.password.clone(),
    };

    match usecase.execute(request).await {
        Ok(response) => HttpResponse::Created().json(RegisterResponseDto {
            user_id: response.user_id,
            username: response.username,
            email: response.email,
        }),
        Err(err) => {
            let (status, code, message) = match err {
                RegisterUserError::UsernameValidation(msg)
                | RegisterUserError::EmailValidation(msg)
                | RegisterUserError::PasswordValidation(msg) => (
                    actix_web::http::StatusCode::BAD_REQUEST,
                    "VALIDATION_ERROR",
                    msg,
                ),
                RegisterUserError::UsernameAlreadyExists => (
                    actix_web::http::StatusCode::CONFLICT,
                    "USERNAME_EXISTS",
                    "Username already exists".to_string(),
                ),
                RegisterUserError::EmailAlreadyExists => (
                    actix_web::http::StatusCode::CONFLICT,
                    "EMAIL_EXISTS",
                    "Email already exists".to_string(),
                ),
                RegisterUserError::Repository(msg) => (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "REPOSITORY_ERROR",
                    msg,
                ),
            };

            HttpResponse::build(status).json(ErrorResponseDto {
                error: message,
                error_code: code.to_string(),
            })
        }
    }
}

/// 認証ルートを設定
pub fn configure_auth_routes<U: UserRepository + 'static, R: RefreshTokenRepository + 'static>(
    cfg: &mut web::ServiceConfig,
) {
    cfg.service(
        web::scope("/api/auth")
            .route("/register", web::post().to(register::<U>))
            .route("/login", web::post().to(login::<U, R>))
            .route("/refresh", web::post().to(refresh::<U, R>))
            .route("/logout", web::post().to(logout::<U, R>)),
    );
}
