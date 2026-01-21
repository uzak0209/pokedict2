use serde::{Deserialize, Serialize};

/// Pokemon Species（種族）エンティティ
/// PokeAPIの pokemon-species に対応
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSpecies {
    /// 図鑑番号（species_id）
    pub species_id: i32,
    /// 種族名（英語）
    pub name: String,
    /// 種族名（日本語）
    pub name_ja: Option<String>,
    /// 伝説フラグ
    pub is_legendary: bool,
    /// 幻フラグ
    pub is_mythical: bool,
}

impl PokemonSpecies {
    #[must_use]
    pub fn new(
        species_id: i32,
        name: String,
        name_ja: Option<String>,
        is_legendary: bool,
        is_mythical: bool,
    ) -> Self {
        Self {
            species_id,
            name,
            name_ja,
            is_legendary,
            is_mythical,
        }
    }
}
