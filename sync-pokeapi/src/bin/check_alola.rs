use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::args()
        .nth(1)
        .expect("Usage: check_alola <DATABASE_URL>");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("=== Checking Ninetales forms ===\n");

    let rows: Vec<(i32, String)> = sqlx::query_as(
        "SELECT form_id, fullname FROM pokemon_forms WHERE fullname LIKE '%ninetales%' ORDER BY fullname"
    )
    .fetch_all(&pool)
    .await?;

    for (form_id, name) in rows {
        println!("  form_id: {}, fullname: {}", form_id, name);
    }

    Ok(())
}
