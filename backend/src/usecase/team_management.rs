use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entity::pokemon_form::PokemonForm;
use crate::domain::entity::team::{Team, TeamError};
use crate::domain::valueobject::teamname::{TeamName, TeamNameValidationError};
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
}

impl<R: TeamRepository> TeamManagementUseCase<R> {
    /// 新しいユースケースインスタンスを作成
    #[must_use]
    pub fn new(team_repository: Arc<R>) -> Self {
        Self { team_repository }
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
            let pokemon = pokemon_data_to_forms(pokemon_data)?;
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
            })
            .collect(),
    }
}

/// PokemonDataをPokemonFormに変換
///
/// TODO: 将来的には以下の実装に変更する
/// 1. PokemonRepositoryでpokemon_nameからform_idを解決
/// 2. PokemonRepositoryからform_idでマスタデータを取得
/// 3. マスタデータから fullname, fullname_jp, species_id, typeset を取得
/// 4. クライアントから受け取った対戦情報（EV, IV, 性格など）と組み合わせる
fn pokemon_data_to_forms(data: Vec<PokemonData>) -> Result<Vec<PokemonForm>, TeamManagementError> {
    use crate::domain::valueobject::move_slot::{Move, MoveSet};
    use crate::domain::valueobject::nature::Nature;
    use crate::domain::valueobject::pokemontype::PokemonType;
    use crate::domain::valueobject::stats::Stats;
    use crate::domain::valueobject::typeset::TypeSet;

    data.into_iter()
        .map(|p| {
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

            // TODO: ここでPokemonRepositoryでpokemon_nameからform_idを解決
            // 現在は暫定的にダミーデータを使用
            let form_id = resolve_pokemon_name_to_form_id(&p.pokemon_name)?;
            let fullname = p.pokemon_name.clone();
            let fullname_jp = p.pokemon_name.clone(); // TODO: マスタデータから取得
            let species_id = form_id; // TODO: マスタデータから取得
            let typeset = TypeSet::new(PokemonType::Normal, None); // TODO: マスタデータから取得

            Ok(PokemonForm::new(
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
            ))
        })
        .collect()
}

/// ポケモン名からform_idを解決する
///
/// TODO: 将来的にはPokemonRepositoryを使用して実装
/// 現在は暫定的にハッシュマップで管理
fn resolve_pokemon_name_to_form_id(pokemon_name: &str) -> Result<i32, TeamManagementError> {
    // TODO: データベースから取得する実装に変更
    // 暫定的なマッピング
    let mapping = [
        ("Pikachu", 25),
        ("Charizard", 6),
        ("Greninja", 658),
        ("Rotom-Wash", 10009),
        ("Rotom-Heat", 10010),
        ("Rotom-Frost", 10011),
        ("Rotom-Fan", 10012),
        ("Rotom-Mow", 10013),
        ("Landorus-Therian", 10019),
        ("Urshifu-Rapid-Strike", 10230),
    ];

    mapping
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(pokemon_name))
        .map(|(_, id)| *id)
        .ok_or_else(|| {
            TeamManagementError::TeamError(format!("Unknown pokemon name: {pokemon_name}"))
        })
}
