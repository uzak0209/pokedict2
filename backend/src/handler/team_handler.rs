use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::repository::team_repository::TeamRepository;
use crate::usecase::team_management::{
    CreateTeamRequest, PokemonData, TeamManagementError, TeamManagementUseCase, UpdateTeamRequest,
};

/// チーム作成リクエストDTO
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamRequestDto {
    pub owner_id: String,
    pub team_name: String,
}

/// チーム作成レスポンスDTO
#[derive(Debug, Serialize)]
pub struct CreateTeamResponseDto {
    pub team_id: String,
    pub owner_id: String,
    pub team_name: String,
}

/// チームレスポンスDTO
#[derive(Debug, Serialize)]
pub struct TeamResponseDto {
    pub team_id: String,
    pub owner_id: String,
    pub team_name: String,
    pub pokemon: Vec<PokemonResponseDto>,
}

/// ポケモンレスポンスDTO
#[derive(Debug, Serialize)]
pub struct PokemonResponseDto {
    pub fullname: String,
    pub fullname_jp: String,
    pub form_id: i32,
    pub species_id: i32,
}

/// チーム更新リクエストDTO
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTeamRequestDto {
    pub team_name: Option<String>,
    pub pokemon: Option<Vec<PokemonDataDto>>,
}

/// ポケモンデータDTO
#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonDataDto {
    pub fullname: String,
    pub fullname_jp: String,
    pub form_id: i32,
    pub species_id: i32,
    pub primary_type: String,
    pub secondary_type: Option<String>,
}

/// エラーレスポンスDTO
#[derive(Debug, Serialize)]
pub struct ErrorResponseDto {
    pub error: String,
}

/// チーム作成ハンドラー
///
/// POST /api/teams
pub async fn create_team<R: TeamRepository + 'static>(
    req: web::Json<CreateTeamRequestDto>,
    repository: web::Data<Arc<R>>,
) -> impl Responder {
    let usecase = TeamManagementUseCase::new(repository.get_ref().clone());

    let request = CreateTeamRequest {
        owner_id: req.owner_id.clone(),
        team_name: req.team_name.clone(),
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
pub async fn get_team<R: TeamRepository + 'static>(
    team_id: web::Path<String>,
    repository: web::Data<Arc<R>>,
) -> impl Responder {
    let usecase = TeamManagementUseCase::new(repository.get_ref().clone());

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
    user_id: web::Path<String>,
    repository: web::Data<Arc<R>>,
) -> impl Responder {
    let usecase = TeamManagementUseCase::new(repository.get_ref().clone());

    match usecase.get_user_teams(&user_id).await {
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
    team_id: web::Path<String>,
    req: web::Json<UpdateTeamRequestDto>,
    repository: web::Data<Arc<R>>,
    // TODO: 認証ミドルウェアからユーザーIDを取得
    // 今は仮でクエリパラメータから取得
    query: web::Query<RequesterQuery>,
) -> impl Responder {
    let usecase = TeamManagementUseCase::new(repository.get_ref().clone());

    let pokemon_data = req.pokemon.as_ref().map(|pokemon| {
        pokemon
            .iter()
            .map(|p| PokemonData {
                fullname: p.fullname.clone(),
                fullname_jp: p.fullname_jp.clone(),
                form_id: p.form_id,
                species_id: p.species_id,
                primary_type: p.primary_type.clone(),
                secondary_type: p.secondary_type.clone(),
            })
            .collect()
    });

    let request = UpdateTeamRequest {
        team_id: team_id.to_string(),
        team_name: req.team_name.clone(),
        pokemon: pokemon_data,
    };

    match usecase.update_team(request, &query.requester_id).await {
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
    team_id: web::Path<String>,
    repository: web::Data<Arc<R>>,
    // TODO: 認証ミドルウェアからユーザーIDを取得
    query: web::Query<RequesterQuery>,
) -> impl Responder {
    let usecase = TeamManagementUseCase::new(repository.get_ref().clone());

    match usecase.delete_team(&team_id, &query.requester_id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(err) => handle_team_error(err),
    }
}

/// リクエスターのクエリパラメータ（仮）
#[derive(Debug, Deserialize)]
pub struct RequesterQuery {
    pub requester_id: String,
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

    HttpResponse::build(status).json(ErrorResponseDto { error: message })
}

/// チーム関連のルートを設定
pub fn configure_team_routes<R: TeamRepository + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/teams")
                    .route("", web::post().to(create_team::<R>))
                    .route("/{team_id}", web::get().to(get_team::<R>))
                    .route("/{team_id}", web::put().to(update_team::<R>))
                    .route("/{team_id}", web::delete().to(delete_team::<R>)),
            )
            .service(
                web::scope("/users").route("/{user_id}/teams", web::get().to(get_user_teams::<R>)),
            ),
    );
}
