use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use typeshare::typeshare;

use crate::handler::auth_middleware::require_auth;
use crate::repository::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
use crate::repository::postgres_user_repository::PostgresUserRepository;
use crate::repository::user_pokemon_repository::UserPokemonRepository;
use crate::usecase::auth_service::AuthService;
use crate::usecase::user_pokemon_management::{
    CreatePokemonRequest, UpdatePokemonRequest, UserPokemonManagementError,
    UserPokemonManagementUseCase,
};

/// ポケモン登録リクエストDTO
#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePokemonRequestDto {
    pub pokemon_name: String,
    pub pokemon_name_jp: String,
    pub nickname: Option<String>,
    pub terastal_type: String,
    pub ev_hp: u16,
    pub ev_attack: u16,
    pub ev_defense: u16,
    pub ev_special_attack: u16,
    pub ev_special_defense: u16,
    pub ev_speed: u16,
    pub iv_hp: u16,
    pub iv_attack: u16,
    pub iv_defense: u16,
    pub iv_special_attack: u16,
    pub iv_special_defense: u16,
    pub iv_speed: u16,
    pub nature: String,
    pub ability: String,
    pub held_item: Option<String>,
    pub moves: Vec<String>,
}

/// ポケモン登録レスポンスDTO
#[typeshare]
#[derive(Debug, Serialize)]
pub struct CreatePokemonResponseDto {
    pub pokemon_id: String,
    pub user_id: String,
    pub nickname: Option<String>,
    pub form_id: i32,
    pub species_id: i32,
    pub fullname: String,
    pub fullname_jp: String,
}

/// ポケモンレスポンスDTO
#[typeshare]
#[derive(Debug, Serialize)]
pub struct PokemonResponseDto {
    pub pokemon_id: String,
    pub user_id: String,
    pub nickname: Option<String>,
    pub form_id: i32,
    pub species_id: i32,
    pub fullname: String,
    pub fullname_jp: String,
    pub terastal_type: String,
    pub ev_hp: u16,
    pub ev_attack: u16,
    pub ev_defense: u16,
    pub ev_special_attack: u16,
    pub ev_special_defense: u16,
    pub ev_speed: u16,
    pub iv_hp: u16,
    pub iv_attack: u16,
    pub iv_defense: u16,
    pub iv_special_attack: u16,
    pub iv_special_defense: u16,
    pub iv_speed: u16,
    pub nature: String,
    pub ability: String,
    pub held_item: Option<String>,
    pub moves: Vec<String>,
}

/// ポケモン更新リクエストDTO
#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePokemonRequestDto {
    pub nickname: Option<String>,
    pub terastal_type: String,
    pub ev_hp: u16,
    pub ev_attack: u16,
    pub ev_defense: u16,
    pub ev_special_attack: u16,
    pub ev_special_defense: u16,
    pub ev_speed: u16,
    pub iv_hp: u16,
    pub iv_attack: u16,
    pub iv_defense: u16,
    pub iv_special_attack: u16,
    pub iv_special_defense: u16,
    pub iv_speed: u16,
    pub nature: String,
    pub ability: String,
    pub held_item: Option<String>,
    pub moves: Vec<String>,
}

/// エラーレスポンスDTO
#[typeshare]
#[derive(Debug, Serialize)]
pub struct PokemonErrorResponseDto {
    pub error: String,
}

/// ポケモン登録ハンドラー
///
/// POST /api/pokemon
/// 認証必須: JWTからuser_idを取得
pub async fn create_pokemon(
    http_req: HttpRequest,
    req: web::Json<CreatePokemonRequestDto>,
    repository: web::Data<Arc<crate::repository::postgres_user_pokemon_repository::PostgresUserPokemonRepository>>,
    auth_service: web::Data<Arc<AuthService<PostgresUserRepository, PostgresRefreshTokenRepository>>>,
) -> impl Responder {
    // JWTからユーザーIDを取得
    let user_id = match require_auth(&http_req, &auth_service).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    let usecase = UserPokemonManagementUseCase::new(repository.get_ref().clone());

    let request = CreatePokemonRequest {
        user_id: user_id.to_string(),
        pokemon_name: req.pokemon_name.clone(),
        pokemon_name_jp: req.pokemon_name_jp.clone(),
        nickname: req.nickname.clone(),
        terastal_type: req.terastal_type.clone(),
        ev_hp: req.ev_hp,
        ev_attack: req.ev_attack,
        ev_defense: req.ev_defense,
        ev_special_attack: req.ev_special_attack,
        ev_special_defense: req.ev_special_defense,
        ev_speed: req.ev_speed,
        iv_hp: req.iv_hp,
        iv_attack: req.iv_attack,
        iv_defense: req.iv_defense,
        iv_special_attack: req.iv_special_attack,
        iv_special_defense: req.iv_special_defense,
        iv_speed: req.iv_speed,
        nature: req.nature.clone(),
        ability: req.ability.clone(),
        held_item: req.held_item.clone(),
        moves: req.moves.clone(),
    };

    match usecase.create_pokemon(request).await {
        Ok(response) => HttpResponse::Created().json(CreatePokemonResponseDto {
            pokemon_id: response.pokemon_id,
            user_id: response.user_id,
            nickname: response.nickname,
            form_id: response.form_id,
            species_id: response.species_id,
            fullname: response.fullname,
            fullname_jp: response.fullname_jp,
        }),
        Err(err) => handle_pokemon_error(err),
    }
}

/// ポケモン取得ハンドラー
///
/// GET /api/pokemon/{pokemon_id}
pub async fn get_pokemon<R: UserPokemonRepository + 'static>(
    pokemon_id: web::Path<String>,
    repository: web::Data<Arc<R>>,
) -> impl Responder {
    let usecase = UserPokemonManagementUseCase::new(repository.get_ref().clone());

    match usecase.get_pokemon(&pokemon_id).await {
        Ok(response) => HttpResponse::Ok().json(PokemonResponseDto {
            pokemon_id: response.pokemon_id,
            user_id: response.user_id,
            nickname: response.nickname,
            form_id: response.form_id,
            species_id: response.species_id,
            fullname: response.fullname,
            fullname_jp: response.fullname_jp,
            terastal_type: response.terastal_type,
            ev_hp: response.ev_hp,
            ev_attack: response.ev_attack,
            ev_defense: response.ev_defense,
            ev_special_attack: response.ev_special_attack,
            ev_special_defense: response.ev_special_defense,
            ev_speed: response.ev_speed,
            iv_hp: response.iv_hp,
            iv_attack: response.iv_attack,
            iv_defense: response.iv_defense,
            iv_special_attack: response.iv_special_attack,
            iv_special_defense: response.iv_special_defense,
            iv_speed: response.iv_speed,
            nature: response.nature,
            ability: response.ability,
            held_item: response.held_item,
            moves: response.moves,
        }),
        Err(err) => handle_pokemon_error(err),
    }
}

/// ユーザーのポケモン一覧取得ハンドラー
///
/// GET /api/pokemon
/// 認証必須: JWTからuser_idを取得
pub async fn get_user_pokemon(
    http_req: HttpRequest,
    repository: web::Data<Arc<crate::repository::postgres_user_pokemon_repository::PostgresUserPokemonRepository>>,
    auth_service: web::Data<Arc<AuthService<PostgresUserRepository, PostgresRefreshTokenRepository>>>,
) -> impl Responder {
    // JWTからユーザーIDを取得
    let user_id = match require_auth(&http_req, &auth_service).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    let usecase = UserPokemonManagementUseCase::new(repository.get_ref().clone());

    match usecase.get_user_pokemon(&user_id.to_string()).await {
        Ok(pokemon_list) => {
            let pokemon_dto: Vec<PokemonResponseDto> = pokemon_list
                .into_iter()
                .map(|p| PokemonResponseDto {
                    pokemon_id: p.pokemon_id,
                    user_id: p.user_id,
                    nickname: p.nickname,
                    form_id: p.form_id,
                    species_id: p.species_id,
                    fullname: p.fullname,
                    fullname_jp: p.fullname_jp,
                    terastal_type: p.terastal_type,
                    ev_hp: p.ev_hp,
                    ev_attack: p.ev_attack,
                    ev_defense: p.ev_defense,
                    ev_special_attack: p.ev_special_attack,
                    ev_special_defense: p.ev_special_defense,
                    ev_speed: p.ev_speed,
                    iv_hp: p.iv_hp,
                    iv_attack: p.iv_attack,
                    iv_defense: p.iv_defense,
                    iv_special_attack: p.iv_special_attack,
                    iv_special_defense: p.iv_special_defense,
                    iv_speed: p.iv_speed,
                    nature: p.nature,
                    ability: p.ability,
                    held_item: p.held_item,
                    moves: p.moves,
                })
                .collect();
            HttpResponse::Ok().json(pokemon_dto)
        }
        Err(err) => handle_pokemon_error(err),
    }
}

/// ポケモン更新ハンドラー
///
/// PUT /api/pokemon/{pokemon_id}
/// 認証必須: JWTからuser_idを取得
pub async fn update_pokemon(
    http_req: HttpRequest,
    pokemon_id: web::Path<String>,
    req: web::Json<UpdatePokemonRequestDto>,
    repository: web::Data<Arc<crate::repository::postgres_user_pokemon_repository::PostgresUserPokemonRepository>>,
    auth_service: web::Data<Arc<AuthService<PostgresUserRepository, PostgresRefreshTokenRepository>>>,
) -> impl Responder {
    // JWTからユーザーIDを取得
    let user_id = match require_auth(&http_req, &auth_service).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    let usecase = UserPokemonManagementUseCase::new(repository.get_ref().clone());

    let request = UpdatePokemonRequest {
        pokemon_id: pokemon_id.to_string(),
        nickname: req.nickname.clone(),
        terastal_type: req.terastal_type.clone(),
        ev_hp: req.ev_hp,
        ev_attack: req.ev_attack,
        ev_defense: req.ev_defense,
        ev_special_attack: req.ev_special_attack,
        ev_special_defense: req.ev_special_defense,
        ev_speed: req.ev_speed,
        iv_hp: req.iv_hp,
        iv_attack: req.iv_attack,
        iv_defense: req.iv_defense,
        iv_special_attack: req.iv_special_attack,
        iv_special_defense: req.iv_special_defense,
        iv_speed: req.iv_speed,
        nature: req.nature.clone(),
        ability: req.ability.clone(),
        held_item: req.held_item.clone(),
        moves: req.moves.clone(),
    };

    match usecase.update_pokemon(request, &user_id.to_string()).await {
        Ok(response) => HttpResponse::Ok().json(PokemonResponseDto {
            pokemon_id: response.pokemon_id,
            user_id: response.user_id,
            nickname: response.nickname,
            form_id: response.form_id,
            species_id: response.species_id,
            fullname: response.fullname,
            fullname_jp: response.fullname_jp,
            terastal_type: response.terastal_type,
            ev_hp: response.ev_hp,
            ev_attack: response.ev_attack,
            ev_defense: response.ev_defense,
            ev_special_attack: response.ev_special_attack,
            ev_special_defense: response.ev_special_defense,
            ev_speed: response.ev_speed,
            iv_hp: response.iv_hp,
            iv_attack: response.iv_attack,
            iv_defense: response.iv_defense,
            iv_special_attack: response.iv_special_attack,
            iv_special_defense: response.iv_special_defense,
            iv_speed: response.iv_speed,
            nature: response.nature,
            ability: response.ability,
            held_item: response.held_item,
            moves: response.moves,
        }),
        Err(err) => handle_pokemon_error(err),
    }
}

/// ポケモン削除ハンドラー
///
/// DELETE /api/pokemon/{pokemon_id}
/// 認証必須: JWTからuser_idを取得
pub async fn delete_pokemon(
    http_req: HttpRequest,
    pokemon_id: web::Path<String>,
    repository: web::Data<Arc<crate::repository::postgres_user_pokemon_repository::PostgresUserPokemonRepository>>,
    auth_service: web::Data<Arc<AuthService<PostgresUserRepository, PostgresRefreshTokenRepository>>>,
) -> impl Responder {
    // JWTからユーザーIDを取得
    let user_id = match require_auth(&http_req, &auth_service).await {
        Ok(id) => id,
        Err(response) => return response,
    };

    let usecase = UserPokemonManagementUseCase::new(repository.get_ref().clone());

    match usecase.delete_pokemon(&pokemon_id, &user_id.to_string()).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(err) => handle_pokemon_error(err),
    }
}

/// エラーハンドリング
fn handle_pokemon_error(err: UserPokemonManagementError) -> HttpResponse {
    let (status, message) = match err {
        UserPokemonManagementError::InvalidUserId | UserPokemonManagementError::InvalidPokemonId => {
            (actix_web::http::StatusCode::BAD_REQUEST, err.to_string())
        }
        UserPokemonManagementError::PokemonNotFound => {
            (actix_web::http::StatusCode::NOT_FOUND, err.to_string())
        }
        UserPokemonManagementError::Unauthorized => {
            (actix_web::http::StatusCode::FORBIDDEN, err.to_string())
        }
        UserPokemonManagementError::InvalidData(msg) => {
            (actix_web::http::StatusCode::BAD_REQUEST, msg)
        }
        UserPokemonManagementError::Repository(msg) => {
            (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
        }
    };

    HttpResponse::build(status).json(PokemonErrorResponseDto { error: message })
}

/// ポケモン関連のルートを設定
pub fn configure_pokemon_routes<R: UserPokemonRepository + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/pokemon")
            // JWT auth required for these endpoints
            .route("", web::post().to(create_pokemon))
            .route("", web::get().to(get_user_pokemon))
            .route("/{pokemon_id}", web::get().to(get_pokemon::<R>))
            .route("/{pokemon_id}", web::put().to(update_pokemon))
            .route("/{pokemon_id}", web::delete().to(delete_pokemon)),
    );
}
