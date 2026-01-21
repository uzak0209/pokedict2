use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use typeshare::typeshare;

use crate::handler::auth_middleware::require_auth;
use crate::repository::postgres_refresh_token_repository::PostgresRefreshTokenRepository;
use crate::repository::postgres_user_repository::PostgresUserRepository;
use crate::repository::team_repository::TeamRepository;
use crate::usecase::auth_service::AuthService;
use crate::usecase::team_management::{
    CreateTeamRequest, PokemonData, TeamManagementError, TeamManagementUseCase, UpdateTeamRequest,
};

/// チーム作成リクエストDTO
#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamRequestDto {
    pub team_name: String,
}

/// チーム作成レスポンスDTO
#[typeshare]
#[derive(Debug, Serialize)]
pub struct CreateTeamResponseDto {
    pub team_id: String,
    pub owner_id: String,
    pub team_name: String,
}

/// チームレスポンスDTO
#[typeshare]
#[derive(Debug, Serialize)]
pub struct TeamResponseDto {
    pub team_id: String,
    pub owner_id: String,
    pub team_name: String,
    pub pokemon: Vec<PokemonResponseDto>,
}

/// ポケモンレスポンスDTO
#[typeshare]
#[derive(Debug, Serialize)]
pub struct PokemonResponseDto {
    pub fullname: String,
    pub fullname_jp: String,
    pub form_id: i32,
    pub species_id: i32,
}

/// チーム更新リクエストDTO
#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTeamRequestDto {
    pub team_name: Option<String>,
    pub pokemon: Option<Vec<PokemonDataDto>>,
}

/// ポケモンデータDTO
#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonDataDto {
    pub pokemon_name: String, // フォルム名を含む完全な名前（例: "Pikachu", "Rotom-Wash"）
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
    pub moves: Vec<String>, // 技名のリスト（最大4つ）
}

/// エラーレスポンスDTO (Team用)
#[typeshare]
#[derive(Debug, Serialize)]
pub struct TeamErrorResponseDto {
    pub error: String,
}

use crate::repository::postgres_pokemon_master_repository::PokemonMasterRepository;

/// チーム作成ハンドラー
///
/// POST /api/teams
pub async fn create_team<R: TeamRepository + 'static>(
    req: HttpRequest,
    body: web::Json<CreateTeamRequestDto>,
    repository: web::Data<Arc<R>>,
    pokemon_repository: web::Data<PokemonMasterRepository>,
    auth_service: web::Data<
        Arc<AuthService<PostgresUserRepository, PostgresRefreshTokenRepository>>,
    >,
) -> impl Responder {
    let user_id = match require_auth(&req, &auth_service).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let usecase = TeamManagementUseCase::new(
        repository.get_ref().clone(),
        pokemon_repository.get_ref().clone(),
    );

    let request = CreateTeamRequest {
        owner_id: user_id.to_string(),
        team_name: body.team_name.clone(),
    };

    match usecase.create_team(request).await {
        Ok(response) => HttpResponse::Created().json(CreateTeamResponseDto {
            team_id: response.team_id,
            owner_id: response.owner_id,
            team_name: response.team_name,
        }),
        Err(err) => handle_team_error(err),
    }
}

/// チーム取得ハンドラー
///
/// GET /api/teams/{team_id}
/// 認証必須にして所有者のみ見せるか、公開範囲設定によるが、一旦認証必須にする
pub async fn get_team<R: TeamRepository + 'static>(
    req: HttpRequest,
    team_id: web::Path<String>,
    repository: web::Data<Arc<R>>,
    pokemon_repository: web::Data<PokemonMasterRepository>,
    auth_service: web::Data<
        Arc<AuthService<PostgresUserRepository, PostgresRefreshTokenRepository>>,
    >,
) -> impl Responder {
    let _user_id = match require_auth(&req, &auth_service).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let usecase = TeamManagementUseCase::new(
        repository.get_ref().clone(),
        pokemon_repository.get_ref().clone(),
    );

    match usecase.get_team(&team_id).await {
        Ok(response) => HttpResponse::Ok().json(TeamResponseDto {
            team_id: response.team_id,
            owner_id: response.owner_id,
            team_name: response.team_name,
            pokemon: response
                .pokemon
                .into_iter()
                .map(|p| PokemonResponseDto {
                    fullname: p.fullname,
                    fullname_jp: p.fullname_jp,
                    form_id: p.form_id,
                    species_id: p.species_id,
                })
                .collect(),
        }),
        Err(err) => handle_team_error(err),
    }
}

/// ユーザーのチーム一覧取得ハンドラー
///
/// GET /api/users/{user_id}/teams
pub async fn get_user_teams<R: TeamRepository + 'static>(
    req: HttpRequest,
    path_user_id: web::Path<String>,
    repository: web::Data<Arc<R>>,
    pokemon_repository: web::Data<PokemonMasterRepository>,
    auth_service: web::Data<
        Arc<AuthService<PostgresUserRepository, PostgresRefreshTokenRepository>>,
    >,
) -> impl Responder {
    let _auth_user_id = match require_auth(&req, &auth_service).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let usecase = TeamManagementUseCase::new(
        repository.get_ref().clone(),
        pokemon_repository.get_ref().clone(),
    );

    match usecase.get_user_teams(&path_user_id).await {
        Ok(teams) => {
            let teams_dto: Vec<TeamResponseDto> = teams
                .into_iter()
                .map(|t| TeamResponseDto {
                    team_id: t.team_id,
                    owner_id: t.owner_id,
                    team_name: t.team_name,
                    pokemon: t
                        .pokemon
                        .into_iter()
                        .map(|p| PokemonResponseDto {
                            fullname: p.fullname,
                            fullname_jp: p.fullname_jp,
                            form_id: p.form_id,
                            species_id: p.species_id,
                        })
                        .collect(),
                })
                .collect();
            HttpResponse::Ok().json(teams_dto)
        }
        Err(err) => handle_team_error(err),
    }
}

/// チーム更新ハンドラー
///
/// PUT /api/teams/{team_id}
pub async fn update_team<R: TeamRepository + 'static>(
    req: HttpRequest,
    team_id: web::Path<String>,
    body: web::Json<UpdateTeamRequestDto>,
    repository: web::Data<Arc<R>>,
    pokemon_repository: web::Data<PokemonMasterRepository>,
    auth_service: web::Data<
        Arc<AuthService<PostgresUserRepository, PostgresRefreshTokenRepository>>,
    >,
) -> impl Responder {
    let user_id = match require_auth(&req, &auth_service).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let usecase = TeamManagementUseCase::new(
        repository.get_ref().clone(),
        pokemon_repository.get_ref().clone(),
    );

    let pokemon_data = body.pokemon.as_ref().map(|pokemon| {
        pokemon
            .iter()
            .map(|p| PokemonData {
                pokemon_name: p.pokemon_name.clone(),
                terastal_type: p.terastal_type.clone(),
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
                nature: p.nature.clone(),
                ability: p.ability.clone(),
                held_item: p.held_item.clone(),
                moves: p.moves.clone(),
            })
            .collect()
    });

    let request = UpdateTeamRequest {
        team_id: team_id.to_string(),
        team_name: body.team_name.clone(),
        pokemon: pokemon_data,
    };

    match usecase.update_team(request, &user_id.to_string()).await {
        Ok(response) => HttpResponse::Ok().json(TeamResponseDto {
            team_id: response.team_id,
            owner_id: response.owner_id,
            team_name: response.team_name,
            pokemon: response
                .pokemon
                .into_iter()
                .map(|p| PokemonResponseDto {
                    fullname: p.fullname,
                    fullname_jp: p.fullname_jp,
                    form_id: p.form_id,
                    species_id: p.species_id,
                })
                .collect(),
        }),
        Err(err) => handle_team_error(err),
    }
}

/// チーム削除ハンドラー
///
/// DELETE /api/teams/{team_id}
pub async fn delete_team<R: TeamRepository + 'static>(
    req: HttpRequest,
    team_id: web::Path<String>,
    repository: web::Data<Arc<R>>,
    pokemon_repository: web::Data<PokemonMasterRepository>,
    auth_service: web::Data<
        Arc<AuthService<PostgresUserRepository, PostgresRefreshTokenRepository>>,
    >,
) -> impl Responder {
    let user_id = match require_auth(&req, &auth_service).await {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let usecase = TeamManagementUseCase::new(
        repository.get_ref().clone(),
        pokemon_repository.get_ref().clone(),
    );

    match usecase.delete_team(&team_id, &user_id.to_string()).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(err) => handle_team_error(err),
    }
}

/// エラーハンドリング
fn handle_team_error(err: TeamManagementError) -> HttpResponse {
    let (status, message) = match err {
        TeamManagementError::TeamNameValidation(msg) => {
            (actix_web::http::StatusCode::BAD_REQUEST, msg)
        }
        TeamManagementError::InvalidOwnerId | TeamManagementError::InvalidTeamId => {
            (actix_web::http::StatusCode::BAD_REQUEST, err.to_string())
        }
        TeamManagementError::TeamNotFound => {
            (actix_web::http::StatusCode::NOT_FOUND, err.to_string())
        }
        TeamManagementError::Unauthorized => {
            (actix_web::http::StatusCode::FORBIDDEN, err.to_string())
        }
        TeamManagementError::TeamError(msg) => (actix_web::http::StatusCode::BAD_REQUEST, msg),
        TeamManagementError::Repository(msg) => {
            (actix_web::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
        }
    };

    HttpResponse::build(status).json(TeamErrorResponseDto { error: message })
}

/// チーム関連のルートを設定
pub fn configure_team_routes<R: TeamRepository + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/teams")
            .route("", web::post().to(create_team::<R>))
            .route("/{team_id}", web::get().to(get_team::<R>))
            .route("/{team_id}", web::put().to(update_team::<R>))
            .route("/{team_id}", web::delete().to(delete_team::<R>)),
    );
    cfg.route(
        "/api/users/{user_id}/teams",
        web::get().to(get_user_teams::<R>),
    );
}
