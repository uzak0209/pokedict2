use serde::{Deserialize, Serialize};

/// チームとポケモンの関連（中間テーブル）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamPokemon {
    /// チームID
    pub team_id: i32,
    /// ポケモンID（user_pokemonへの参照）
    pub pokemon_id: i32,
    /// スロット位置（1-6）
    pub slot: i32,
}

impl TeamPokemon {
    #[must_use]
    pub fn new(team_id: i32, pokemon_id: i32, slot: i32) -> Self {
        Self {
            team_id,
            pokemon_id,
            slot,
        }
    }
}
