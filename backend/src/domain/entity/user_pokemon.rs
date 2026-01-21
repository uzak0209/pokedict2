use crate::domain::valueobject::move_slot::MoveSet;
use crate::domain::valueobject::nature::Nature;
use crate::domain::valueobject::pokemontype::PokemonType;
use crate::domain::valueobject::stats::Stats;
use crate::domain::valueobject::typeset::TypeSet;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// ユーザーが保存する個別のポケモン
#[derive(Debug, Clone)]
pub struct UserPokemon {
    pokemon_id: Uuid,
    user_id: Uuid,
    nickname: Option<String>,

    // マスタデータから取得する情報
    form_id: i32,
    species_id: i32,
    fullname: String,
    fullname_jp: String,
    typeset: TypeSet,

    // ユーザーが設定する対戦情報
    terastal_type: PokemonType,
    ev: Stats,
    iv: Stats,
    nature: Nature,
    ability: String,
    held_item: Option<String>,
    moves: MoveSet,

    created_at: DateTime<Utc>,
}

impl UserPokemon {
    /// 新しいユーザーポケモンを作成
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: Uuid,
        nickname: Option<String>,
        form_id: i32,
        species_id: i32,
        fullname: String,
        fullname_jp: String,
        typeset: TypeSet,
        terastal_type: PokemonType,
        ev: Stats,
        iv: Stats,
        nature: Nature,
        ability: String,
        held_item: Option<String>,
        moves: MoveSet,
    ) -> Self {
        Self {
            pokemon_id: Uuid::new_v4(),
            user_id,
            nickname,
            form_id,
            species_id,
            fullname,
            fullname_jp,
            typeset,
            terastal_type,
            ev,
            iv,
            nature,
            ability,
            held_item,
            moves,
            created_at: Utc::now(),
        }
    }

    /// DBから復元する際に使用
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn from_repository(
        pokemon_id: Uuid,
        user_id: Uuid,
        nickname: Option<String>,
        form_id: i32,
        species_id: i32,
        fullname: String,
        fullname_jp: String,
        typeset: TypeSet,
        terastal_type: PokemonType,
        ev: Stats,
        iv: Stats,
        nature: Nature,
        ability: String,
        held_item: Option<String>,
        moves: MoveSet,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            pokemon_id,
            user_id,
            nickname,
            form_id,
            species_id,
            fullname,
            fullname_jp,
            typeset,
            terastal_type,
            ev,
            iv,
            nature,
            ability,
            held_item,
            moves,
            created_at,
        }
    }

    /// ポケモンIDを取得
    #[must_use]
    pub fn pokemon_id(&self) -> &Uuid {
        &self.pokemon_id
    }

    /// ユーザーIDを取得
    #[must_use]
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    /// ニックネームを取得
    #[must_use]
    pub fn nickname(&self) -> Option<&str> {
        self.nickname.as_deref()
    }

    /// フォームIDを取得
    #[must_use]
    pub fn form_id(&self) -> i32 {
        self.form_id
    }

    /// 種族IDを取得
    #[must_use]
    pub fn species_id(&self) -> i32 {
        self.species_id
    }

    /// フルネーム（英語）を取得
    #[must_use]
    pub fn fullname(&self) -> &str {
        &self.fullname
    }

    /// フルネーム（日本語）を取得
    #[must_use]
    pub fn fullname_jp(&self) -> &str {
        &self.fullname_jp
    }

    /// タイプセットを取得
    #[must_use]
    pub fn typeset(&self) -> &TypeSet {
        &self.typeset
    }

    /// テラスタルタイプを取得
    #[must_use]
    pub fn terastal_type(&self) -> &PokemonType {
        &self.terastal_type
    }

    /// 努力値を取得
    #[must_use]
    pub fn ev(&self) -> &Stats {
        &self.ev
    }

    /// 個体値を取得
    #[must_use]
    pub fn iv(&self) -> &Stats {
        &self.iv
    }

    /// 性格を取得
    #[must_use]
    pub fn nature(&self) -> &Nature {
        &self.nature
    }

    /// 特性を取得
    #[must_use]
    pub fn ability(&self) -> &str {
        &self.ability
    }

    /// 持ち物を取得
    #[must_use]
    pub fn held_item(&self) -> Option<&str> {
        self.held_item.as_deref()
    }

    /// 技セットを取得
    #[must_use]
    pub fn moves(&self) -> &MoveSet {
        &self.moves
    }

    /// 作成日時を取得
    #[must_use]
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    /// ニックネームを更新
    pub fn update_nickname(&mut self, nickname: Option<String>) {
        self.nickname = nickname;
    }

    /// 対戦情報を更新
    #[allow(clippy::too_many_arguments)]
    pub fn update_battle_info(
        &mut self,
        terastal_type: PokemonType,
        ev: Stats,
        iv: Stats,
        nature: Nature,
        ability: String,
        held_item: Option<String>,
        moves: MoveSet,
    ) {
        self.terastal_type = terastal_type;
        self.ev = ev;
        self.iv = iv;
        self.nature = nature;
        self.ability = ability;
        self.held_item = held_item;
        self.moves = moves;
    }
}
