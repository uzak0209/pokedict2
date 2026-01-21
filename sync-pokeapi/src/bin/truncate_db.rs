use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::args()
        .nth(1)
        .expect("Usage: truncate_db <DATABASE_URL>");

    println!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("⚠️  TRUNCATING TABLES...");

    // 外部キーの依存順に削除
    println!("- Truncating usage_stats...");
    sqlx::query("TRUNCATE TABLE usage_stats CASCADE")
        .execute(&pool)
        .await?;

    println!("- Truncating pokemon_forms...");
    sqlx::query("TRUNCATE TABLE pokemon_forms CASCADE")
        .execute(&pool)
        .await?;

    println!("- Truncating pokemon_species...");
    sqlx::query("TRUNCATE TABLE pokemon_species CASCADE")
        .execute(&pool)
        .await?;

    println!("✅ All tables truncated successfully.");
    Ok(())
}
