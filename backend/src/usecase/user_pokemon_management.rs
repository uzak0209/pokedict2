use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entity::user_pokemon::UserPokemon;
use crate::domain::valueobject::move_slot::{Move, MoveSet};
use crate::domain::valueobject::nature::Nature;
use crate::domain::valueobject::pokemontype::PokemonType;
use crate::domain::valueobject::stats::Stats;
use crate::domain::valueobject::typeset::TypeSet;
use crate::repository::user_pokemon_repository::{UserPokemonRepository, UserPokemonRepositoryError};

/// ポケモン登録リクエスト
#[derive(Debug, Clone)]
pub struct CreatePokemonRequest {
    pub user_id: String,
    pub pokemon_name: String, // フォルム名を含む完全な名前
    pub pokemon_name_jp: String, // 日本語名
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

/// ポケモン登録レスポンス
#[derive(Debug, Clone)]
pub struct CreatePokemonResponse {
    pub pokemon_id: String,
    pub user_id: String,
    pub nickname: Option<String>,
    pub form_id: i32,
    pub species_id: i32,
    pub fullname: String,
    pub fullname_jp: String,
}

/// ポケモン取得レスポンス
#[derive(Debug, Clone)]
pub struct PokemonResponse {
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

/// ポケモン更新リクエスト
#[derive(Debug, Clone)]
pub struct UpdatePokemonRequest {
    pub pokemon_id: String,
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

/// ユーザーポケモン管理のエラー
#[derive(Debug, thiserror::Error)]
pub enum UserPokemonManagementError {
    #[error("Invalid user ID")]
    InvalidUserId,
    #[error("Invalid pokemon ID")]
    InvalidPokemonId,
    #[error("Pokemon not found")]
    PokemonNotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    Repository(String),
}

impl From<UserPokemonRepositoryError> for UserPokemonManagementError {
    fn from(err: UserPokemonRepositoryError) -> Self {
        match err {
            UserPokemonRepositoryError::NotFound => Self::PokemonNotFound,
            UserPokemonRepositoryError::DatabaseError(msg) => Self::Repository(msg),
        }
    }
}

/// ユーザーポケモン管理のユースケース
pub struct UserPokemonManagementUseCase<R: UserPokemonRepository> {
    pokemon_repository: Arc<R>,
}

impl<R: UserPokemonRepository> UserPokemonManagementUseCase<R> {
    /// 新しいユースケースインスタンスを作成
    #[must_use]
    pub fn new(pokemon_repository: Arc<R>) -> Self {
        Self { pokemon_repository }
    }

    /// ポケモンを登録
    ///
    /// # Errors
    ///
    /// - バリデーションエラーやリポジトリエラーが発生した場合
    pub async fn create_pokemon(
        &self,
        request: CreatePokemonRequest,
    ) -> Result<CreatePokemonResponse, UserPokemonManagementError> {
        // 1. ユーザーIDのパース
        let user_id = Uuid::parse_str(&request.user_id)
            .map_err(|_| UserPokemonManagementError::InvalidUserId)?;

        // 2. テラスタルタイプのパース
        let terastal_type = request.terastal_type.parse::<PokemonType>()
            .map_err(|_| UserPokemonManagementError::InvalidData(format!("Invalid terastal type: {}", request.terastal_type)))?;

        // 3. 努力値の作成
        let ev = Stats::new_ev(
            request.ev_hp,
            request.ev_attack,
            request.ev_defense,
            request.ev_special_attack,
            request.ev_special_defense,
            request.ev_speed,
        ).map_err(|e| UserPokemonManagementError::InvalidData(format!("Invalid EV: {e}")))?;

        // 4. 個体値の作成
        let iv = Stats::new_iv(
            request.iv_hp,
            request.iv_attack,
            request.iv_defense,
            request.iv_special_attack,
            request.iv_special_defense,
            request.iv_speed,
        ).map_err(|e| UserPokemonManagementError::InvalidData(format!("Invalid IV: {e}")))?;

        // 5. 性格のパース
        let nature = request.nature.parse::<Nature>()
            .map_err(|_| UserPokemonManagementError::InvalidData(format!("Invalid nature: {}", request.nature)))?;

        // 6. 技の変換
        let moves_vec: Result<Vec<Move>, _> = request.moves.iter()
            .map(|move_name| Move::new(move_name)
                .map_err(|e| UserPokemonManagementError::InvalidData(format!("Invalid move: {e}"))))
            .collect();
        let moves = MoveSet::from_vec(moves_vec?)
            .map_err(|e| UserPokemonManagementError::InvalidData(format!("Invalid move set: {e}")))?;

        // 7. ポケモン名からform_idを解決
        let form_id = resolve_pokemon_name_to_form_id(&request.pokemon_name)?;

        // フロントエンドから受け取った名前を使用
        let fullname = request.pokemon_name.clone();
        let fullname_jp = request.pokemon_name_jp.clone();
        let species_id = form_id;
        let typeset = TypeSet::new(PokemonType::Normal, None);

        // 8. UserPokemonエンティティの作成
        let pokemon = UserPokemon::new(
            user_id,
            request.nickname.clone(),
            form_id,
            species_id,
            fullname.clone(),
            fullname_jp.clone(),
            typeset,
            terastal_type,
            ev,
            iv,
            nature,
            request.ability.clone(),
            request.held_item.clone(),
            moves,
        );

        // 9. リポジトリに保存
        self.pokemon_repository.save(&pokemon).await?;

        // 10. レスポンスを返す
        Ok(CreatePokemonResponse {
            pokemon_id: pokemon.pokemon_id().to_string(),
            user_id: pokemon.user_id().to_string(),
            nickname: request.nickname,
            form_id,
            species_id,
            fullname,
            fullname_jp,
        })
    }

    /// ポケモンを取得
    ///
    /// # Errors
    ///
    /// - ポケモンIDが不正な場合
    /// - ポケモンが見つからない場合
    pub async fn get_pokemon(
        &self,
        pokemon_id: &str,
    ) -> Result<PokemonResponse, UserPokemonManagementError> {
        let pokemon_id = Uuid::parse_str(pokemon_id)
            .map_err(|_| UserPokemonManagementError::InvalidPokemonId)?;

        let pokemon = self
            .pokemon_repository
            .find_by_id(&pokemon_id)
            .await?
            .ok_or(UserPokemonManagementError::PokemonNotFound)?;

        Ok(pokemon_to_response(&pokemon))
    }

    /// ユーザーの全ポケモンを取得
    ///
    /// # Errors
    ///
    /// - ユーザーIDが不正な場合
    pub async fn get_user_pokemon(
        &self,
        user_id: &str,
    ) -> Result<Vec<PokemonResponse>, UserPokemonManagementError> {
        let user_id = Uuid::parse_str(user_id)
            .map_err(|_| UserPokemonManagementError::InvalidUserId)?;

        let pokemon_list = self.pokemon_repository.find_by_user_id(&user_id).await?;

        Ok(pokemon_list.iter().map(pokemon_to_response).collect())
    }

    /// ポケモンを更新
    ///
    /// # Errors
    ///
    /// - ポケモンIDが不正な場合
    /// - ポケモンが見つからない場合
    /// - バリデーションエラーが発生した場合
    pub async fn update_pokemon(
        &self,
        request: UpdatePokemonRequest,
        requester_id: &str,
    ) -> Result<PokemonResponse, UserPokemonManagementError> {
        let pokemon_id = Uuid::parse_str(&request.pokemon_id)
            .map_err(|_| UserPokemonManagementError::InvalidPokemonId)?;

        let mut pokemon = self
            .pokemon_repository
            .find_by_id(&pokemon_id)
            .await?
            .ok_or(UserPokemonManagementError::PokemonNotFound)?;

        // 権限チェック
        let requester_uuid = Uuid::parse_str(requester_id)
            .map_err(|_| UserPokemonManagementError::InvalidUserId)?;
        if pokemon.user_id() != &requester_uuid {
            return Err(UserPokemonManagementError::Unauthorized);
        }

        // データの更新
        let terastal_type = request.terastal_type.parse::<PokemonType>()
            .map_err(|_| UserPokemonManagementError::InvalidData(format!("Invalid terastal type: {}", request.terastal_type)))?;

        let ev = Stats::new_ev(
            request.ev_hp,
            request.ev_attack,
            request.ev_defense,
            request.ev_special_attack,
            request.ev_special_defense,
            request.ev_speed,
        ).map_err(|e| UserPokemonManagementError::InvalidData(format!("Invalid EV: {e}")))?;

        let iv = Stats::new_iv(
            request.iv_hp,
            request.iv_attack,
            request.iv_defense,
            request.iv_special_attack,
            request.iv_special_defense,
            request.iv_speed,
        ).map_err(|e| UserPokemonManagementError::InvalidData(format!("Invalid IV: {e}")))?;

        let nature = request.nature.parse::<Nature>()
            .map_err(|_| UserPokemonManagementError::InvalidData(format!("Invalid nature: {}", request.nature)))?;

        let moves_vec: Result<Vec<Move>, _> = request.moves.iter()
            .map(|move_name| Move::new(move_name)
                .map_err(|e| UserPokemonManagementError::InvalidData(format!("Invalid move: {e}"))))
            .collect();
        let moves = MoveSet::from_vec(moves_vec?)
            .map_err(|e| UserPokemonManagementError::InvalidData(format!("Invalid move set: {e}")))?;

        pokemon.update_nickname(request.nickname);
        pokemon.update_battle_info(
            terastal_type,
            ev,
            iv,
            nature,
            request.ability,
            request.held_item,
            moves,
        );

        // リポジトリに保存
        self.pokemon_repository.update(&pokemon).await?;

        Ok(pokemon_to_response(&pokemon))
    }

    /// ポケモンを削除
    ///
    /// # Errors
    ///
    /// - ポケモンIDが不正な場合
    /// - ポケモンが見つからない場合
    /// - 権限がない場合
    pub async fn delete_pokemon(
        &self,
        pokemon_id: &str,
        requester_id: &str,
    ) -> Result<(), UserPokemonManagementError> {
        let pokemon_id = Uuid::parse_str(pokemon_id)
            .map_err(|_| UserPokemonManagementError::InvalidPokemonId)?;

        let pokemon = self
            .pokemon_repository
            .find_by_id(&pokemon_id)
            .await?
            .ok_or(UserPokemonManagementError::PokemonNotFound)?;

        // 権限チェック
        let requester_uuid = Uuid::parse_str(requester_id)
            .map_err(|_| UserPokemonManagementError::InvalidUserId)?;
        if pokemon.user_id() != &requester_uuid {
            return Err(UserPokemonManagementError::Unauthorized);
        }

        // ポケモンを削除
        self.pokemon_repository.delete(&pokemon_id).await?;

        Ok(())
    }
}

/// UserPokemonエンティティをPokemonResponseに変換
fn pokemon_to_response(pokemon: &UserPokemon) -> PokemonResponse {
    PokemonResponse {
        pokemon_id: pokemon.pokemon_id().to_string(),
        user_id: pokemon.user_id().to_string(),
        nickname: pokemon.nickname().map(String::from),
        form_id: pokemon.form_id(),
        species_id: pokemon.species_id(),
        fullname: pokemon.fullname().to_string(),
        fullname_jp: pokemon.fullname_jp().to_string(),
        terastal_type: pokemon.terastal_type().to_string(),
        ev_hp: pokemon.ev().hp,
        ev_attack: pokemon.ev().attack,
        ev_defense: pokemon.ev().defense,
        ev_special_attack: pokemon.ev().special_attack,
        ev_special_defense: pokemon.ev().special_defense,
        ev_speed: pokemon.ev().speed,
        iv_hp: pokemon.iv().hp,
        iv_attack: pokemon.iv().attack,
        iv_defense: pokemon.iv().defense,
        iv_special_attack: pokemon.iv().special_attack,
        iv_special_defense: pokemon.iv().special_defense,
        iv_speed: pokemon.iv().speed,
        nature: pokemon.nature().to_string(),
        ability: pokemon.ability().to_string(),
        held_item: pokemon.held_item().map(String::from),
        moves: pokemon.moves().move_list().iter().map(|m| m.name().to_string()).collect(),
    }
}

/// ポケモン名からform_idを解決する
///
/// TODO: 将来的にはPokemonRepositoryを使用して実装
fn resolve_pokemon_name_to_form_id(pokemon_name: &str) -> Result<i32, UserPokemonManagementError> {
    // TODO: データベースから取得する実装に変更
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
            UserPokemonManagementError::InvalidData(format!("Unknown pokemon name: {pokemon_name}"))
        })
}
