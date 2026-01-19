mod domain;
mod handler;
mod repository;
mod usecase;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;

use crate::handler::{team_handler, user_handler};
use crate::repository::mock_team_repository::MockTeamRepository;
use crate::repository::mock_user_repository::MockUserRepository;
use crate::repository::postgres_team_repository::PostgresTeamRepository;
use crate::repository::postgres_user_repository::PostgresUserRepository;

/// ヘルスチェックエンドポイント
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "pokedict2-backend"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ロガーの初期化
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // JWT秘密鍵（本番環境では環境変数から取得すべき）
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        println!("Warning: JWT_SECRET not set, using default secret for development");
        "default_jwt_secret_please_change_in_production".to_string()
    });

    // 環境変数でリポジトリ実装を切り替え
    // USE_POSTGRES=true でPostgreSQL、未設定または false でモック実装
    let use_postgres = std::env::var("USE_POSTGRES")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase() == "true";

    if use_postgres {
        // PostgreSQL実装を使用
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set when USE_POSTGRES=true");

        println!("📦 Connecting to PostgreSQL...");
        let pool = sqlx::PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to PostgreSQL");

        let user_repository = Arc::new(PostgresUserRepository::new(pool.clone()));
        let team_repository = Arc::new(PostgresTeamRepository::new(pool.clone()));

        // マイグレーションを実行
        println!("🔧 Running migrations...");
        user_repository.migrate().await.expect("Failed to migrate users table");
        team_repository.migrate().await.expect("Failed to migrate teams table");
        println!("✅ Migrations completed");

        start_server_with_postgres(user_repository, team_repository, jwt_secret).await
    } else {
        // モック実装を使用（開発/テスト用）
        println!("🧪 Using mock repositories (in-memory)");
        println!("   To use PostgreSQL, set USE_POSTGRES=true and DATABASE_URL");

        let user_repository = Arc::new(MockUserRepository::new());
        let team_repository = Arc::new(MockTeamRepository::new());

        start_server_with_mock(user_repository, team_repository, jwt_secret).await
    }
}

async fn start_server_with_postgres(
    user_repository: Arc<PostgresUserRepository>,
    team_repository: Arc<PostgresTeamRepository>,
    jwt_secret: String,
) -> std::io::Result<()> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{host}:{port}");

    print_startup_message(&bind_address, "PostgreSQL");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(user_repository.clone()))
            .app_data(web::Data::new(team_repository.clone()))
            .app_data(web::Data::new(jwt_secret.clone()))
            .configure(user_handler::configure_auth_routes::<PostgresUserRepository>)
            .configure(team_handler::configure_team_routes::<PostgresTeamRepository>)
            .route("/health", web::get().to(health_check))
    })
    .bind(&bind_address)?
    .run()
    .await
}

async fn start_server_with_mock(
    user_repository: Arc<MockUserRepository>,
    team_repository: Arc<MockTeamRepository>,
    jwt_secret: String,
) -> std::io::Result<()> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{host}:{port}");

    print_startup_message(&bind_address, "Mock (In-Memory)");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(user_repository.clone()))
            .app_data(web::Data::new(team_repository.clone()))
            .app_data(web::Data::new(jwt_secret.clone()))
            .configure(user_handler::configure_auth_routes::<MockUserRepository>)
            .configure(team_handler::configure_team_routes::<MockTeamRepository>)
            .route("/health", web::get().to(health_check))
    })
    .bind(&bind_address)?
    .run()
    .await
}

fn print_startup_message(bind_address: &str, repository_type: &str) {
    println!("🚀 Starting server at http://{bind_address}");
    println!("💾 Repository: {repository_type}");
    println!("📖 API Documentation:");
    println!("   === Auth ===");
    println!("   POST   /api/auth/register      - ユーザー登録");
    println!("   POST   /api/auth/login         - ログイン");
    println!("   === Teams ===");
    println!("   POST   /api/teams              - チーム作成");
    println!("   GET    /api/teams/{{team_id}}    - チーム取得");
    println!("   PUT    /api/teams/{{team_id}}    - チーム更新");
    println!("   DELETE /api/teams/{{team_id}}    - チーム削除");
    println!("   GET    /api/users/{{user_id}}/teams - ユーザーのチーム一覧");
    println!("   === Other ===");
    println!("   GET    /health                 - ヘルスチェック");
}
