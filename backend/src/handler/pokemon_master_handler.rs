use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::repository::postgres_pokemon_master_repository::{
    PokemonMasterRepository, PokemonMasterResponseDto, PokemonUsageStatsDto,
};

/// 全ポケモンマスタデータ取得ハンドラー
///
/// GET /api/pokemon/master
pub async fn get_all_pokemon_master(
    pool: web::Data<PgPool>,
) -> impl Responder {
    let repo = PokemonMasterRepository::new(pool.get_ref().clone());

    match repo.get_all_pokemon().await {
        Ok(pokemon) => {
            let total = pokemon.len() as i32;
            HttpResponse::Ok().json(PokemonMasterResponseDto { pokemon, total })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to fetch pokemon: {}", e),
                "error_code": "DATABASE_ERROR"
            }))
        }
    }
}

/// 使用率トップポケモン取得ハンドラー
///
/// GET /api/pokemon/master/top?limit=50
pub async fn get_top_pokemon_master(
    pool: web::Data<PgPool>,
    query: web::Query<TopPokemonQuery>,
) -> impl Responder {
    let repo = PokemonMasterRepository::new(pool.get_ref().clone());
    let limit = query.limit.unwrap_or(50).min(500);

    match repo.get_top_pokemon(limit).await {
        Ok(pokemon) => {
            let total = pokemon.len() as i32;
            HttpResponse::Ok().json(PokemonMasterResponseDto { pokemon, total })
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to fetch pokemon: {}", e),
                "error_code": "DATABASE_ERROR"
            }))
        }
    }
}

/// ポケモン使用率統計取得ハンドラー
///
/// GET /api/pokemon/master/{form_id}/usage
pub async fn get_pokemon_usage(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let form_id = path.into_inner();
    let repo = PokemonMasterRepository::new(pool.get_ref().clone());

    match repo.get_usage_stats(form_id).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => {
            // データが見つからない場合も空を返すか、エラーにするか。
            // ここでは500エラーとしてログに残す
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to fetch usage stats: {}", e),
                "error_code": "DATABASE_ERROR"
            }))
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct TopPokemonQuery {
    pub limit: Option<i32>,
}

/// ポケモンマスタデータのルートを設定
pub fn configure_pokemon_master_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/pokemon/master")
            .route("", web::get().to(get_all_pokemon_master))
            .route("/top", web::get().to(get_top_pokemon_master))
            .route("/{form_id}/usage", web::get().to(get_pokemon_usage)),
    );
}

