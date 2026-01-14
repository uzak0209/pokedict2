use super::effective::Effectiveness;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PokemonType {
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Normal,
    Ghost,
    Dragon,
    Ice,
    Fighting,
    Flying,
    Bug,
    Rock,
    Ground,
    Poison,
    Steel,
    Fairy,
    Dark,
}

impl PokemonType {
    /// 全てのポケモンタイプを配列で返す
    /// Returns all Pokemon types
    #[must_use]
    pub fn all_types() -> [Self; 18] {
        [
            Self::Normal,
            Self::Fire,
            Self::Water,
            Self::Grass,
            Self::Electric,
            Self::Ice,
            Self::Fighting,
            Self::Poison,
            Self::Ground,
            Self::Flying,
            Self::Psychic,
            Self::Bug,
            Self::Rock,
            Self::Ghost,
            Self::Dragon,
            Self::Dark,
            Self::Steel,
            Self::Fairy,
        ]
    }

    /// 攻撃側のタイプ(self)が防御側のタイプ(defender)に対する相性を返す
    /// Returns the effectiveness of this attacking type against the defending type
    #[allow(clippy::match_same_arms)]
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn effectiveness_against(&self, defender: &PokemonType) -> Effectiveness {
        use PokemonType::{
            Bug, Dark, Dragon, Electric, Fairy, Fighting, Fire, Flying, Ghost, Grass, Ground, Ice,
            Normal, Poison, Psychic, Rock, Steel, Water,
        };

        match (self, defender) {
            // Normal
            (Normal, Rock | Steel) => Effectiveness::Half,
            (Normal, Ghost) => Effectiveness::NoEffect,

            // Fire
            (Fire, Fire | Water | Rock | Dragon) => Effectiveness::Half,
            (Fire, Grass | Ice | Bug | Steel) => Effectiveness::Double,

            // Water
            (Water, Water | Grass | Dragon) => Effectiveness::Half,
            (Water, Fire | Ground | Rock) => Effectiveness::Double,

            // Grass
            (Grass, Fire | Grass | Poison | Flying | Bug | Dragon | Steel) => Effectiveness::Half,
            (Grass, Water | Ground | Rock) => Effectiveness::Double,

            // Electric
            (Electric, Electric | Grass | Dragon) => Effectiveness::Half,
            (Electric, Ground) => Effectiveness::NoEffect,
            (Electric, Water | Flying) => Effectiveness::Double,

            // Ice
            (Ice, Fire | Water | Ice | Steel) => Effectiveness::Half,
            (Ice, Grass | Ground | Flying | Dragon) => Effectiveness::Double,

            // Fighting
            (Fighting, Poison | Flying | Psychic | Bug | Fairy) => Effectiveness::Half,
            (Fighting, Ghost) => Effectiveness::NoEffect,
            (Fighting, Normal | Ice | Rock | Dark | Steel) => Effectiveness::Double,

            // Poison
            (Poison, Poison | Ground | Rock | Ghost) => Effectiveness::Half,
            (Poison, Steel) => Effectiveness::NoEffect,
            (Poison, Grass | Fairy) => Effectiveness::Double,

            // Ground
            (Ground, Grass | Bug) => Effectiveness::Half,
            (Ground, Flying) => Effectiveness::NoEffect,
            (Ground, Fire | Electric | Poison | Rock | Steel) => Effectiveness::Double,

            // Flying
            (Flying, Electric | Rock | Steel) => Effectiveness::Half,
            (Flying, Grass | Fighting | Bug) => Effectiveness::Double,

            // Psychic
            (Psychic, Psychic | Steel) => Effectiveness::Half,
            (Psychic, Dark) => Effectiveness::NoEffect,
            (Psychic, Fighting | Poison) => Effectiveness::Double,

            // Bug
            (Bug, Fire | Fighting | Poison | Flying | Ghost | Steel | Fairy) => Effectiveness::Half,
            (Bug, Grass | Psychic | Dark) => Effectiveness::Double,

            // Rock
            (Rock, Fighting | Ground | Steel) => Effectiveness::Half,
            (Rock, Fire | Ice | Flying | Bug) => Effectiveness::Double,

            // Ghost
            (Ghost, Dark) => Effectiveness::Half,
            (Ghost, Normal) => Effectiveness::NoEffect,
            (Ghost, Psychic | Ghost) => Effectiveness::Double,

            // Dragon
            (Dragon, Steel) => Effectiveness::Half,
            (Dragon, Fairy) => Effectiveness::NoEffect,
            (Dragon, Dragon) => Effectiveness::Double,

            // Dark
            (Dark, Fighting | Dark | Fairy) => Effectiveness::Half,
            (Dark, Psychic | Ghost) => Effectiveness::Double,

            // Steel
            (Steel, Fire | Water | Electric | Steel) => Effectiveness::Half,
            (Steel, Ice | Rock | Fairy) => Effectiveness::Double,

            // Fairy
            (Fairy, Fire | Poison | Steel) => Effectiveness::Half,
            (Fairy, Fighting | Dragon | Dark) => Effectiveness::Double,

            // デフォルトは等倍
            _ => Effectiveness::Neutral,
        }
    }
}
