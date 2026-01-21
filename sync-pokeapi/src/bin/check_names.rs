use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: check_names <DATABASE_URL>");
        return Ok(());
    }
    let db_url = &args[1];

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    println!("=== Checking form_name and fullname stats ===");

    // Count empty form_name
    let empty_form_name: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM pokemon_forms WHERE form_name IS NULL OR form_name = ''",
    )
    .fetch_one(&pool)
    .await?;
    println!("Rows with empty/null form_name: {}", empty_form_name.0);

    // Count empty fullname
    let empty_fullname: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM pokemon_forms WHERE fullname IS NULL OR fullname = ''",
    )
    .fetch_one(&pool)
    .await?;
    println!("Rows with empty/null fullname: {}", empty_fullname.0);

    // Count empty fullname_ja
    let empty_fullname_ja: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM pokemon_forms WHERE fullname_ja IS NULL OR fullname_ja = ''",
    )
    .fetch_one(&pool)
    .await?;
    println!("Rows with empty/null fullname_ja: {}", empty_fullname_ja.0);

    println!("\n=== Checking Ninetales forms ===");
    let ninetales_rows: Vec<(i32, String, Option<String>)> = sqlx::query_as(
        r#"
        SELECT f.form_id, f.fullname, f.fullname_ja 
        FROM pokemon_forms f
        WHERE f.fullname LIKE '%ninetales%'
        ORDER BY f.form_id ASC
        "#,
    )
    .fetch_all(&pool)
    .await?;

    for row in ninetales_rows {
        println!(
            "  ID: {}, Fullname: {}, Fullname_ja: {:?}",
            row.0, row.1, row.2
        );
    }

    println!("\n=== Checking Arceus forms ===");
    let arceus_rows: Vec<(i32, String, Option<String>)> = sqlx::query_as(
        r#"
        SELECT f.form_id, f.fullname, f.fullname_ja 
        FROM pokemon_forms f
        WHERE f.fullname LIKE '%arceus%'
        ORDER BY f.form_id ASC
        LIMIT 5
        "#,
    )
    .fetch_all(&pool)
    .await?;

    for row in arceus_rows {
        println!(
            "  ID: {}, Fullname: {}, Fullname_ja: {:?}",
            row.0, row.1, row.2
        );
    }

    Ok(())
}
