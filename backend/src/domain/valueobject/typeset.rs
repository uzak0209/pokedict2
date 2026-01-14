use crate::domain::valueobject::effective::Effectiveness;
use crate::domain::valueobject::pokemontype::PokemonType;

#[derive(Debug)]
pub struct TypeSet {
    primary: PokemonType,
    secondary: Option<PokemonType>,
}

impl TypeSet {
    pub fn new(primary: PokemonType, secondary: Option<PokemonType>) -> Self {
        TypeSet { primary, secondary }
    }

    pub fn primary(&self) -> &PokemonType {
        &self.primary
    }

    pub fn secondary(&self) -> Option<&PokemonType> {
        self.secondary.as_ref()
    }

    pub fn has_secondary(&self) -> bool {
        self.secondary.is_some()
    }

    /// 攻撃側のタイプから受けるダメージの効果を計算
    /// Calculates the effectiveness of an attacking type against this `TypeSet`
    ///
    /// # Examples
    ///
    /// ```
    /// use pokedict2::domain::valueobject::pokemontype::PokemonType;
    /// use pokedict2::domain::valueobject::typeset::TypeSet;
    ///
    /// // Water type attacks against Fire type
    /// let fire_type = TypeSet::new(PokemonType::Fire, None);
    /// let effectiveness = fire_type.defend_against(&PokemonType::Water);
    /// // Returns Effectiveness::Double (2.0x damage)
    ///
    /// // Electric attacks against Water/Flying type
    /// let gyarados = TypeSet::new(PokemonType::Water, Some(PokemonType::Flying));
    /// let effectiveness = gyarados.defend_against(&PokemonType::Electric);
    /// // Returns Effectiveness::Quadruple (4.0x damage)
    /// ```
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn defend_against(&self, attacking_type: &PokemonType) -> Effectiveness {
        // プライマリータイプに対する相性を計算
        let primary_effectiveness = attacking_type.effectiveness_against(&self.primary);

        // セカンダリータイプがある場合は、その相性も計算して掛け合わせる
        match &self.secondary {
            Some(secondary_type) => {
                let secondary_effectiveness = attacking_type.effectiveness_against(secondary_type);

                // 両方の効果を掛け合わせる
                let combined_multiplier = primary_effectiveness.get_multiplier_tocalc()
                    * secondary_effectiveness.get_multiplier_tocalc()
                    / 4; // 4で割るのは、Neutral (4) * Neutral (4) = 16 となり、これをNeutral (4)にするため

                Effectiveness::from_multiplier(combined_multiplier)
            }
            None => primary_effectiveness,
        }
    }

    /// 複数の攻撃タイプに対する効果を一度に計算
    /// Calculates effectiveness against multiple attacking types at once
    pub fn defend_against_multiple(
        &self,
        attacking_types: &[PokemonType],
    ) -> Vec<(PokemonType, Effectiveness)> {
        attacking_types
            .iter()
            .map(|attacking_type| (*attacking_type, self.defend_against(attacking_type)))
            .collect()
    }

    /// 全ての攻撃タイプに対する効果を一度に計算
    /// Calculates effectiveness against all Pokemon types at once
    ///
    /// # Examples
    ///
    /// ```
    /// use pokedict2::domain::valueobject::pokemontype::PokemonType;
    /// use pokedict2::domain::valueobject::typeset::TypeSet;
    ///
    /// // Get all type effectiveness for Fire type
    /// let fire_type = TypeSet::new(PokemonType::Fire, None);
    /// let all_effectiveness = fire_type.defend_against_all();
    ///
    /// // Returns 18 entries, one for each type
    /// assert_eq!(all_effectiveness.len(), 18);
    /// ```
    #[must_use]
    pub fn defend_against_all(&self) -> Vec<(PokemonType, Effectiveness)> {
        PokemonType::all_types()
            .iter()
            .map(|attacking_type| (*attacking_type, self.defend_against(attacking_type)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_type_super_effective() {
        // 水タイプが炎タイプに効果抜群（2倍）
        let fire_type = TypeSet::new(PokemonType::Fire, None);
        let effectiveness = fire_type.defend_against(&PokemonType::Water);
        assert_eq!(effectiveness, Effectiveness::Double);
        assert_eq!(effectiveness.multiplier(), 2.0);
    }

    #[test]
    fn test_single_type_not_very_effective() {
        // 水タイプが草タイプに今一つ（0.5倍）
        let grass_type = TypeSet::new(PokemonType::Grass, None);
        let effectiveness = grass_type.defend_against(&PokemonType::Water);
        assert_eq!(effectiveness, Effectiveness::Half);
        assert_eq!(effectiveness.multiplier(), 0.5);
    }

    #[test]
    fn test_single_type_no_effect() {
        // ノーマルタイプがゴーストタイプに無効（0倍）
        let ghost_type = TypeSet::new(PokemonType::Ghost, None);
        let effectiveness = ghost_type.defend_against(&PokemonType::Normal);
        assert_eq!(effectiveness, Effectiveness::NoEffect);
        assert_eq!(effectiveness.multiplier(), 0.0);
    }

    #[test]
    fn test_dual_type_quadruple_damage() {
        // 電気タイプが水/飛行タイプ（ギャラドスなど）に4倍ダメージ
        let water_flying = TypeSet::new(PokemonType::Water, Some(PokemonType::Flying));
        let effectiveness = water_flying.defend_against(&PokemonType::Electric);
        assert_eq!(effectiveness, Effectiveness::Quadruple);
        assert_eq!(effectiveness.multiplier(), 4.0);
    }

    #[test]
    fn test_dual_type_quarter_damage() {
        // 草タイプが鋼/飛行タイプ（エアームドなど）に1/4ダメージ
        let steel_flying = TypeSet::new(PokemonType::Steel, Some(PokemonType::Flying));
        let effectiveness = steel_flying.defend_against(&PokemonType::Grass);
        assert_eq!(effectiveness, Effectiveness::Quarter);
        assert_eq!(effectiveness.multiplier(), 0.25);
    }

    #[test]
    fn test_dual_type_with_immunity() {
        // 地面タイプが岩/飛行タイプ（プテラなど）に無効（飛行タイプが地面を無効化）
        let rock_flying = TypeSet::new(PokemonType::Rock, Some(PokemonType::Flying));
        let effectiveness = rock_flying.defend_against(&PokemonType::Ground);
        assert_eq!(effectiveness, Effectiveness::NoEffect);
        assert_eq!(effectiveness.multiplier(), 0.0);
    }

    #[test]
    fn test_dual_type_neutral() {
        // ノーマルタイプが水/地面タイプに等倍ダメージ
        let water_ground = TypeSet::new(PokemonType::Water, Some(PokemonType::Ground));
        let effectiveness = water_ground.defend_against(&PokemonType::Normal);
        assert_eq!(effectiveness, Effectiveness::Neutral);
        assert_eq!(effectiveness.multiplier(), 1.0);
    }

    #[test]
    fn test_defend_against_multiple() {
        let fire_type = TypeSet::new(PokemonType::Fire, None);
        let attacking_types = vec![PokemonType::Water, PokemonType::Grass, PokemonType::Fire];

        let results = fire_type.defend_against_multiple(&attacking_types);

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].1, Effectiveness::Double); // Water -> Fire
        assert_eq!(results[1].1, Effectiveness::Half); // Grass -> Fire
        assert_eq!(results[2].1, Effectiveness::Half); // Fire -> Fire
    }

    #[test]
    fn test_defend_against_all() {
        let fire_type = TypeSet::new(PokemonType::Fire, None);
        let all_results = fire_type.defend_against_all();

        // 全18タイプが返されることを確認
        assert_eq!(all_results.len(), 18);

        // いくつかの相性を確認
        let water_effectiveness =
            all_results.iter().find(|(t, _)| *t == PokemonType::Water).map(|(_, e)| *e);
        assert_eq!(water_effectiveness, Some(Effectiveness::Double));

        let grass_effectiveness =
            all_results.iter().find(|(t, _)| *t == PokemonType::Grass).map(|(_, e)| *e);
        assert_eq!(grass_effectiveness, Some(Effectiveness::Half));
    }

    #[test]
    fn test_all_types_returns_18_types() {
        let all_types = PokemonType::all_types();
        assert_eq!(all_types.len(), 18);

        // 全タイプがユニークであることを確認
        let mut unique_types = std::collections::HashSet::new();
        for pokemon_type in &all_types {
            unique_types.insert(pokemon_type);
        }
        assert_eq!(unique_types.len(), 18);
    }

    #[test]
    fn test_defend_against_all_dual_type() {
        // 水/飛行タイプ（ギャラドスなど）の全相性を確認
        let water_flying = TypeSet::new(PokemonType::Water, Some(PokemonType::Flying));
        let all_results = water_flying.defend_against_all();

        assert_eq!(all_results.len(), 18);

        // 電気タイプが4倍であることを確認
        let electric_effectiveness =
            all_results.iter().find(|(t, _)| *t == PokemonType::Electric).map(|(_, e)| *e);
        assert_eq!(electric_effectiveness, Some(Effectiveness::Quadruple));

        // 地面タイプが無効であることを確認（飛行タイプが無効化）
        let ground_effectiveness =
            all_results.iter().find(|(t, _)| *t == PokemonType::Ground).map(|(_, e)| *e);
        assert_eq!(ground_effectiveness, Some(Effectiveness::NoEffect));
    }
}
