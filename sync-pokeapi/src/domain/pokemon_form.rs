use serde::{Deserialize, Serialize};

/// Pokemon Form（フォーム）エンティティ
/// PokeAPIの pokemon に対応
/// 各フォームは一意のform_idを持つ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonForm {
    /// フォーム一意ID（pokemon.id）
    pub form_id: i32,
    /// 図鑑番号（species_id）
    pub species_id: i32,
    /// フォーム名（例: "alola", "mega", 通常フォームの場合はNone）
    pub form_name: Option<String>,
    /// フルネーム（例: "pikachu", "pikachu-alola"）
    pub fullname: String,
    /// 日本語名
    pub fullname_ja: Option<String>,
    /// タイプ1
    pub type1: String,
    /// タイプ2（単タイプの場合はNone）
    pub type2: Option<String>,
    /// 種族値
    pub base_stats: BaseStats,
}

/// 種族値
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseStats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub sp_attack: i32,
    pub sp_defense: i32,
    pub speed: i32,
}

impl PokemonForm {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        form_id: i32,
        species_id: i32,
        form_name: Option<String>,
        fullname: String,
        fullname_ja: Option<String>,
        type1: String,
        type2: Option<String>,
        base_stats: BaseStats,
    ) -> Self {
        Self {
            form_id,
            species_id,
            form_name,
            fullname,
            fullname_ja,
            type1,
            type2,
            base_stats,
        }
    }
}

impl BaseStats {
    #[must_use]
    pub fn new(
        hp: i32,
        attack: i32,
        defense: i32,
        sp_attack: i32,
        sp_defense: i32,
        speed: i32,
    ) -> Self {
        Self {
            hp,
            attack,
            defense,
            sp_attack,
            sp_defense,
            speed,
        }
    }
}
