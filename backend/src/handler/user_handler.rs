use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::repository::user_repository::UserRepository;
use crate::usecase::user_login::{LoginError, LoginRequest, UserLoginUseCase};
use crate::usecase::user_registration::{
    RegisterUserError, RegisterUserRequest, UserRegistrationUseCase,
};

/// ユーザー登録のリクエストDTO
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequestDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// ユーザー登録のレスポンスDTO
#[derive(Debug, Serialize)]
pub struct RegisterResponseDto {
    pub user_id: String,
    pub username: String,
    pub email: String,
}

/// ログインリクエストDTO
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestDto {
    pub email: String,
    pub password: String,
}

/// ログインレスポンスDTO
#[derive(Debug, Serialize)]
pub struct LoginResponseDto {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub token: String,
}

/// エラーレスポンスDTO
#[derive(Debug, Serialize)]
pub struct ErrorResponseDto {
    pub error: String,
}

/// ユーザー登録ハンドラー
///
/// POST /api/auth/register
pub async fn register<R: UserRepository + 'static>(
    req: web::Json<RegisterRequestDto>,
    repository: web::Data<Arc<R>>,
) -> impl Responder {
    let usecase = UserRegistrationUseCase::new(repository.get_ref().clone());

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
            let (status, message) = match err {
                RegisterUserError::UsernameValidation(msg)
                | RegisterUserError::EmailValidation(msg)
                | RegisterUserError::PasswordValidation(msg) => {
                    (actix_web::http::StatusCode::BAD_REQUEST, msg)
                }
                RegisterUserError::UsernameAlreadyExists => (
                    actix_web::http::StatusCode::CONFLICT,
                    "Username already exists".to_string(),
                ),
                RegisterUserError::EmailAlreadyExists => (
                    actix_web::http::StatusCode::CONFLICT,
                    "Email already exists".to_string(),
                ),
                RegisterUserError::Repository(msg) => {
                    (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
                }
            };

            HttpResponse::build(status).json(ErrorResponseDto { error: message })
        }
    }
}

/// ログインハンドラー
///
/// POST /api/auth/login
pub async fn login<R: UserRepository + 'static>(
    req: web::Json<LoginRequestDto>,
    repository: web::Data<Arc<R>>,
    jwt_secret: web::Data<String>,
) -> impl Responder {
    let usecase = UserLoginUseCase::new(
        repository.get_ref().clone(),
        jwt_secret.get_ref().clone(),
        24, // トークンの有効期限: 24時間
    );

    let request = LoginRequest {
        email: req.email.clone(),
        password: req.password.clone(),
    };

    match usecase.execute(request).await {
        Ok(response) => HttpResponse::Ok().json(LoginResponseDto {
            user_id: response.user_id,
            username: response.username,
            email: response.email,
            token: response.token,
        }),
        Err(err) => {
            let (status, message) = match err {
                LoginError::InvalidCredentials => (
                    actix_web::http::StatusCode::UNAUTHORIZED,
                    "Invalid email or password".to_string(),
                ),
                LoginError::EmailValidation(msg) => (actix_web::http::StatusCode::BAD_REQUEST, msg),
                LoginError::Repository(msg) | LoginError::TokenGeneration(msg) => {
                    (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
                }
            };

            HttpResponse::build(status).json(ErrorResponseDto { error: message })
        }
    }
}

/// 認証ルートを設定
pub fn configure_auth_routes<R: UserRepository + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("/register", web::post().to(register::<R>))
            .route("/login", web::post().to(login::<R>)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entity::user::User;
    use crate::domain::valueobject::email::Email;
    use crate::domain::valueobject::username::Username;
    use crate::repository::user_repository::UserRepositoryError;
    use actix_web::{test, App};
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use uuid::Uuid;

    // テスト用のインメモリリポジトリ
    struct InMemoryUserRepository {
        users: Mutex<HashMap<Uuid, User>>,
    }

    impl InMemoryUserRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl UserRepository for InMemoryUserRepository {
        async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
            let mut users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            users.insert(*user.user_id(), user.clone());
            Ok(())
        }

        async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, UserRepositoryError> {
            let users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            Ok(users.get(user_id).cloned())
        }

        async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError> {
            let users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            Ok(users.values().find(|u| u.email().as_str() == email.as_str()).cloned())
        }

        async fn find_by_username(
            &self,
            username: &Username,
        ) -> Result<Option<User>, UserRepositoryError> {
            let users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            Ok(users.values().find(|u| u.username().as_str() == username.as_str()).cloned())
        }

        async fn exists_by_email(&self, email: &Email) -> Result<bool, UserRepositoryError> {
            Ok(self.find_by_email(email).await?.is_some())
        }

        async fn exists_by_username(
            &self,
            username: &Username,
        ) -> Result<bool, UserRepositoryError> {
            Ok(self.find_by_username(username).await?.is_some())
        }
    }

    #[actix_web::test]
    #[allow(clippy::unwrap_used)]
    async fn test_register_endpoint() {
        let repository = Arc::new(InMemoryUserRepository::new());
        let jwt_secret = "test_secret".to_string();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(repository.clone()))
                .app_data(web::Data::new(jwt_secret))
                .configure(configure_auth_routes::<InMemoryUserRepository>),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/auth/register")
            .set_json(RegisterRequestDto {
                username: "testuser".to_string(),
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);
    }

    #[actix_web::test]
    #[allow(clippy::unwrap_used)]
    async fn test_login_endpoint() {
        let repository = Arc::new(InMemoryUserRepository::new());
        let jwt_secret = "test_secret".to_string();

        // テストユーザーを事前に登録
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(username, email, "password123").unwrap();
        repository.save(&user).await.unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(repository.clone()))
                .app_data(web::Data::new(jwt_secret))
                .configure(configure_auth_routes::<InMemoryUserRepository>),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/auth/login")
            .set_json(LoginRequestDto {
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }
}
