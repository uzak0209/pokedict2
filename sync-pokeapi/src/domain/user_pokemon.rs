use serde::{Deserialize, Serialize};

/// ユーザーが作成したポケモン構成（チームから独立）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPokemon {
    /// ポケモンID
    pub pokemon_id: i32,
    /// ユーザーID
    pub user_id: String,
    /// ポケモンフォームID
    pub form_id: i32,
    /// ニックネーム（オプション）
    pub nickname: Option<String>,
    /// レベル（デフォルト50）
    pub level: i32,
    /// 性格
    pub nature: String,
    /// 特性
    pub ability: String,
    /// 持ち物（オプション）
    pub item: Option<String>,
    /// テラスタイプ
    pub tera_type: String,
    /// 技1
    pub move1: String,
    /// 技2（オプション）
    pub move2: Option<String>,
    /// 技3（オプション）
    pub move3: Option<String>,
    /// 技4（オプション）
    pub move4: Option<String>,
    /// 努力値: HP
    pub ev_hp: i32,
    /// 努力値: 攻撃
    pub ev_attack: i32,
    /// 努力値: 防御
    pub ev_defense: i32,
    /// 努力値: 特攻
    pub ev_sp_attack: i32,
    /// 努力値: 特防
    pub ev_sp_defense: i32,
    /// 努力値: 素早さ
    pub ev_speed: i32,
    /// 個体値: HP（オプション、デフォルト31）
    pub iv_hp: Option<i32>,
    /// 個体値: 攻撃（オプション、デフォルト31）
    pub iv_attack: Option<i32>,
    /// 個体値: 防御（オプション、デフォルト31）
    pub iv_defense: Option<i32>,
    /// 個体値: 特攻（オプション、デフォルト31）
    pub iv_sp_attack: Option<i32>,
    /// 個体値: 特防（オプション、デフォルト31）
    pub iv_sp_defense: Option<i32>,
    /// 個体値: 素早さ（オプション、デフォルト31）
    pub iv_speed: Option<i32>,
    /// 作成日時
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl UserPokemon {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: String,
        form_id: i32,
        nickname: Option<String>,
        level: i32,
        nature: String,
        ability: String,
        item: Option<String>,
        tera_type: String,
        move1: String,
        move2: Option<String>,
        move3: Option<String>,
        move4: Option<String>,
        ev_hp: i32,
        ev_attack: i32,
        ev_defense: i32,
        ev_sp_attack: i32,
        ev_sp_defense: i32,
        ev_speed: i32,
    ) -> Self {
        Self {
            pokemon_id: 0, // データベースで自動採番
            user_id,
            form_id,
            nickname,
            level,
            nature,
            ability,
            item,
            tera_type,
            move1,
            move2,
            move3,
            move4,
            ev_hp,
            ev_attack,
            ev_defense,
            ev_sp_attack,
            ev_sp_defense,
            ev_speed,
            iv_hp: None,
            iv_attack: None,
            iv_defense: None,
            iv_sp_attack: None,
            iv_sp_defense: None,
            iv_speed: None,
            created_at: None,
        }
    }

    /// 努力値の合計が508以下かチェック
    #[must_use]
    pub fn validate_evs(&self) -> bool {
        let total = self.ev_hp
            + self.ev_attack
            + self.ev_defense
            + self.ev_sp_attack
            + self.ev_sp_defense
            + self.ev_speed;
        total <= 508
    }

    /// 各努力値が252以下かチェック
    #[must_use]
    pub fn validate_individual_evs(&self) -> bool {
        self.ev_hp <= 252
            && self.ev_attack <= 252
            && self.ev_defense <= 252
            && self.ev_sp_attack <= 252
            && self.ev_sp_defense <= 252
            && self.ev_speed <= 252
    }
}
