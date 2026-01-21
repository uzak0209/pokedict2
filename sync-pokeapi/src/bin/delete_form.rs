use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::args()
        .nth(1)
        .expect("Usage: delete_form <DATABASE_URL> <FULLNAME>");
    let fullname = env::args()
        .nth(2)
        .expect("Usage: delete_form <DATABASE_URL> <FULLNAME>");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Deleting usage stats for form: {}", fullname);
    // まずform_idを取得
    let form_id: Option<i32> =
        sqlx::query_scalar("SELECT form_id FROM pokemon_forms WHERE fullname = $1")
            .bind(&fullname)
            .fetch_optional(&pool)
            .await?;

    if let Some(id) = form_id {
        sqlx::query("DELETE FROM usage_stats WHERE form_id = $1")
            .bind(id)
            .execute(&pool)
            .await?;

        println!("Deleting form: {}", fullname);
        let result = sqlx::query("DELETE FROM pokemon_forms WHERE form_id = $1")
            .bind(id)
            .execute(&pool)
            .await?;
        println!("Deleted {} rows", result.rows_affected());
    } else {
        println!("Form not found: {}", fullname);
    }

    // usage_statsも削除しておかないと外部キー制約などが（あれば）引っかかるかも
    // 今回はusage_statsのform_idがcascadeになっていないかもしれないので確認
    // ただsave_usage_statsはon conflict updateなので、formさえあれば大丈夫

    Ok(())
}
