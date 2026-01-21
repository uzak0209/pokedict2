use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entity::pokemon_form::PokemonForm;
use crate::domain::entity::team::{Team, TeamError};
use crate::domain::valueobject::teamname::{TeamName, TeamNameValidationError};
use crate::repository::postgres_pokemon_master_repository::PokemonMasterRepository;
use crate::repository::team_repository::{TeamRepository, TeamRepositoryError};

/// チーム作成リクエスト
#[derive(Debug, Clone)]
pub struct CreateTeamRequest {
    pub owner_id: String,
    pub team_name: String,
}

/// チーム作成レスポンス
#[derive(Debug, Clone)]
pub struct CreateTeamResponse {
    pub team_id: String,
    pub owner_id: String,
    pub team_name: String,
}

/// チーム取得レスポンス
#[derive(Debug, Clone)]
pub struct TeamResponse {
    pub team_id: String,
    pub owner_id: String,
    pub team_name: String,
    pub pokemon: Vec<PokemonResponse>,
}

/// ポケモンレスポンス
#[derive(Debug, Clone)]
pub struct PokemonResponse {
    pub fullname: String,
    pub fullname_jp: String,
    pub form_id: i32,
    pub species_id: i32,
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

/// チーム更新リクエスト
#[derive(Debug, Clone)]
pub struct UpdateTeamRequest {
    pub team_id: String,
    pub team_name: Option<String>,
    pub pokemon: Option<Vec<PokemonData>>,
}

/// ポケモンデータ（クライアントから受け取る対戦情報）
#[derive(Debug, Clone)]
pub struct PokemonData {
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


/// チーム管理のエラー
#[derive(Debug, thiserror::Error)]
pub enum TeamManagementError {
    #[error("Team name validation failed: {0}")]
    TeamNameValidation(String),
    #[error("Invalid owner ID")]
    InvalidOwnerId,
    #[error("Invalid team ID")]
    InvalidTeamId,
    #[error("Team not found")]
    TeamNotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Team error: {0}")]
    TeamError(String),
    #[error("Repository error: {0}")]
    Repository(String),
}

impl From<TeamNameValidationError> for TeamManagementError {
    fn from(err: TeamNameValidationError) -> Self {
        Self::TeamNameValidation(err.to_string())
    }
}

impl From<TeamRepositoryError> for TeamManagementError {
    fn from(err: TeamRepositoryError) -> Self {
        match err {
            TeamRepositoryError::NotFound => Self::TeamNotFound,
            TeamRepositoryError::DatabaseError(msg) => Self::Repository(msg),
        }
    }
}

impl From<TeamError> for TeamManagementError {
    fn from(err: TeamError) -> Self {
        Self::TeamError(err.to_string())
    }
}

/// チーム管理のユースケース
pub struct TeamManagementUseCase<R: TeamRepository> {
    team_repository: Arc<R>,
    pokemon_repository: PokemonMasterRepository,
}

impl<R: TeamRepository> TeamManagementUseCase<R> {
    /// 新しいユースケースインスタンスを作成
    #[must_use]
    pub fn new(team_repository: Arc<R>, pokemon_repository: PokemonMasterRepository) -> Self {
        Self {
            team_repository,
            pokemon_repository,
        }
    }

    /// チームを作成
    ///
    /// # Errors
    ///
    /// - バリデーションエラーやリポジトリエラーが発生した場合
    pub async fn create_team(
        &self,
        request: CreateTeamRequest,
    ) -> Result<CreateTeamResponse, TeamManagementError> {
        // 1. オーナーIDのパース
        let owner_id =
            Uuid::parse_str(&request.owner_id).map_err(|_| TeamManagementError::InvalidOwnerId)?;

        // 2. チーム名のバリデーション
        let team_name = TeamName::new(&request.team_name)?;

        // 3. チームエンティティの作成
        let team = Team::new(owner_id, team_name.clone());

        // 4. リポジトリに保存
        self.team_repository.save(&team).await?;

        // 5. レスポンスを返す
        Ok(CreateTeamResponse {
            team_id: team.team_id().to_string(),
            owner_id: team.owner_id().to_string(),
            team_name: team_name.as_str().to_string(),
        })
    }

    /// チームを取得
    ///
    /// # Errors
    ///
    /// - チームIDが不正な場合
    /// - チームが見つからない場合
    pub async fn get_team(&self, team_id: &str) -> Result<TeamResponse, TeamManagementError> {
        // 1. チームIDのパース
        let team_id = Uuid::parse_str(team_id).map_err(|_| TeamManagementError::InvalidTeamId)?;

        // 2. チームを取得
        let team = self
            .team_repository
            .find_by_id(&team_id)
            .await?
            .ok_or(TeamManagementError::TeamNotFound)?;

        // 3. レスポンスに変換
        Ok(team_to_response(&team))
    }

    /// ユーザーの全チームを取得
    ///
    /// # Errors
    ///
    /// - ユーザーIDが不正な場合
    pub async fn get_user_teams(
        &self,
        owner_id: &str,
    ) -> Result<Vec<TeamResponse>, TeamManagementError> {
        // 1. オーナーIDのパース
        let owner_id =
            Uuid::parse_str(owner_id).map_err(|_| TeamManagementError::InvalidOwnerId)?;

        // 2. チームリストを取得
        let teams = self.team_repository.find_by_owner_id(&owner_id).await?;

        // 3. レスポンスに変換
        Ok(teams.iter().map(team_to_response).collect())
    }

    /// チームを更新
    ///
    /// # Errors
    ///
    /// - チームIDが不正な場合
    /// - チームが見つからない場合
    /// - バリデーションエラーが発生した場合
    pub async fn update_team(
        &self,
        request: UpdateTeamRequest,
        requester_id: &str,
    ) -> Result<TeamResponse, TeamManagementError> {
        // 1. チームIDのパース
        let team_id =
            Uuid::parse_str(&request.team_id).map_err(|_| TeamManagementError::InvalidTeamId)?;

        // 2. チームを取得
        let mut team = self
            .team_repository
            .find_by_id(&team_id)
            .await?
            .ok_or(TeamManagementError::TeamNotFound)?;

        // 3. 権限チェック（リクエスターがオーナーか確認）
        let requester_uuid =
            Uuid::parse_str(requester_id).map_err(|_| TeamManagementError::InvalidOwnerId)?;
        if team.owner_id() != &requester_uuid {
            return Err(TeamManagementError::Unauthorized);
        }

        // 4. チーム名の更新
        if let Some(new_name) = request.team_name {
            let team_name = TeamName::new(&new_name)?;
            team.update_name(team_name);
        }

        // 5. ポケモンの更新
        if let Some(pokemon_data) = request.pokemon {
            let pokemon = self.resolve_pokemon_data(pokemon_data).await?;
            team.update_pokemon(pokemon)?;
        }

        // 6. リポジトリに保存
        self.team_repository.update(&team).await?;

        // 7. レスポンスを返す
        Ok(team_to_response(&team))
    }

    /// チームを削除
    ///
    /// # Errors
    ///
    /// - チームIDが不正な場合
    /// - チームが見つからない場合
    /// - 権限がない場合
    pub async fn delete_team(
        &self,
        team_id: &str,
        requester_id: &str,
    ) -> Result<(), TeamManagementError> {
        // 1. チームIDのパース
        let team_id = Uuid::parse_str(team_id).map_err(|_| TeamManagementError::InvalidTeamId)?;

        // 2. チームを取得
        let team = self
            .team_repository
            .find_by_id(&team_id)
            .await?
            .ok_or(TeamManagementError::TeamNotFound)?;

        // 3. 権限チェック
        let requester_uuid =
            Uuid::parse_str(requester_id).map_err(|_| TeamManagementError::InvalidOwnerId)?;
        if team.owner_id() != &requester_uuid {
            return Err(TeamManagementError::Unauthorized);
        }

        // 4. チームを削除
        self.team_repository.delete(&team_id).await?;

        Ok(())
    }

    /// PokemonDataをPokemonFormに変換（DB解決あり）
    async fn resolve_pokemon_data(
        &self,
        data: Vec<PokemonData>,
    ) -> Result<Vec<PokemonForm>, TeamManagementError> {
        use crate::domain::valueobject::move_slot::{Move, MoveSet};
        use crate::domain::valueobject::nature::Nature;
        use crate::domain::valueobject::pokemontype::PokemonType;
        use crate::domain::valueobject::stats::Stats;
        use crate::domain::valueobject::typeset::TypeSet;

        let mut forms = Vec::new();

        for p in data {
            // テラスタルタイプのパース
            let terastal_type = p.terastal_type.parse::<PokemonType>().map_err(|_| {
                TeamManagementError::TeamError(format!(
                    "Invalid terastal type: {}",
                    p.terastal_type
                ))
            })?;

            // 努力値の作成
            let ev = Stats::new_ev(
                p.ev_hp,
                p.ev_attack,
                p.ev_defense,
                p.ev_special_attack,
                p.ev_special_defense,
                p.ev_speed,
            )
            .map_err(|e| TeamManagementError::TeamError(format!("Invalid EV: {e}")))?;

            // 個体値の作成
            let iv = Stats::new_iv(
                p.iv_hp,
                p.iv_attack,
                p.iv_defense,
                p.iv_special_attack,
                p.iv_special_defense,
                p.iv_speed,
            )
            .map_err(|e| TeamManagementError::TeamError(format!("Invalid IV: {e}")))?;

            // 性格のパース
            let nature = p
                .nature
                .parse::<Nature>()
                .map_err(|_| TeamManagementError::TeamError(format!("Invalid nature: {}", p.nature)))?;

            // 技の変換
            let moves_vec: Result<Vec<Move>, _> = p
                .moves
                .iter()
                .map(|move_name| {
                    Move::new(move_name).map_err(|e| {
                        TeamManagementError::TeamError(format!("Invalid move name: {e}"))
                    })
                })
                .collect();
            let moves = MoveSet::from_vec(moves_vec?).map_err(|e| {
                TeamManagementError::TeamError(format!("Invalid move set: {e}"))
            })?;

            // ポケモン名からform_idとメタデータをDB解決
            let master = self
                .pokemon_repository
                .find_by_name(&p.pokemon_name)
                .await
                .map_err(|e| TeamManagementError::Repository(e.to_string()))?
                .ok_or_else(|| {
                    TeamManagementError::TeamError(format!("Unknown pokemon name: {}", p.pokemon_name))
                })?;

            let form_id = master.form_id;
            let fullname = master.fullname;
            let fullname_jp = master.fullname_ja.unwrap_or_else(|| fullname.clone());
            let species_id = master.species_id;

            // タイプのパース
            let type1 = master
                .type1
                .parse::<PokemonType>()
                .unwrap_or(PokemonType::Normal);
            let type2 = master
                .type2
                .and_then(|t| t.parse::<PokemonType>().ok());
            let typeset = TypeSet::new(type1, type2);

            forms.push(PokemonForm::new(
                form_id,
                species_id,
                fullname,
                fullname_jp,
                typeset,
                terastal_type,
                ev,
                iv,
                nature,
                p.ability,
                p.held_item,
                moves,
            ));
        }

        Ok(forms)
    }
}

/// TeamエンティティをTeamResponseに変換
fn team_to_response(team: &Team) -> TeamResponse {
    TeamResponse {
        team_id: team.team_id().to_string(),
        owner_id: team.owner_id().to_string(),
        team_name: team.team_name().as_str().to_string(),
        pokemon: team
            .pokemon_list()
            .iter()
            .map(|p| PokemonResponse {
                fullname: p.fullname().to_string(),
                fullname_jp: p.fullname_jp().to_string(),
                form_id: p.form_id(),
                species_id: p.species_id(),
                terastal_type: p.terastal_type().to_string(),
                ev_hp: p.ev().hp,
                ev_attack: p.ev().attack,
                ev_defense: p.ev().defense,
                ev_special_attack: p.ev().special_attack,
                ev_special_defense: p.ev().special_defense,
                ev_speed: p.ev().speed,
                iv_hp: p.iv().hp,
                iv_attack: p.iv().attack,
                iv_defense: p.iv().defense,
                iv_special_attack: p.iv().special_attack,
                iv_special_defense: p.iv().special_defense,
                iv_speed: p.iv().speed,
                nature: p.nature().to_string(),
                ability: p.ability().to_string(),
                held_item: p.held_item().map(|s| s.to_string()),
                moves: p.moves().move_list().iter().map(|m| m.name().to_string()).collect(),
            })
            .collect(),
    }
}
