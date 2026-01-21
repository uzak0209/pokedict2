use crate::domain::{BaseStats, PokemonForm, PokemonSpecies};
use crate::form_localization::localize_fullname_ja;
use serde::{Deserialize, Serialize};

/// PokeAPI pokemon-species レスポンス
#[derive(Debug, Deserialize)]
pub struct SpeciesResponse {
    pub id: i32,
    pub name: String,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub names: Vec<NameEntry>,
    pub varieties: Vec<VarietyEntry>,
}

#[derive(Debug, Deserialize)]
pub struct NameEntry {
    pub name: String,
    pub language: Language,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct VarietyEntry {
    #[allow(dead_code)]
    pub is_default: bool,
    pub pokemon: NamedAPIResource,
}

#[derive(Debug, Deserialize)]
pub struct NamedAPIResource {
    pub name: String,
    pub url: String,
}

impl SpeciesResponse {
    /// ドメインモデルに変換
    pub fn into_domain(self) -> PokemonSpecies {
        let name_ja = self
            .names
            .iter()
            .find(|n| n.language.name == "ja" || n.language.name == "ja-Hrkt")
            .map(|n| n.name.clone());

        PokemonSpecies::new(
            self.id,
            self.name,
            name_ja,
            self.is_legendary,
            self.is_mythical,
        )
    }

    /// このspeciesに属する全てのpokemon URLを取得
    pub fn get_pokemon_urls(&self) -> Vec<String> {
        self.varieties
            .iter()
            .map(|v| v.pokemon.url.clone())
            .collect()
    }
}

/// PokeAPI pokemon レスポンス
#[derive(Debug, Deserialize)]
pub struct PokemonResponse {
    pub id: i32,
    pub name: String,
    #[allow(dead_code)]
    pub species: NamedAPIResource,
    pub types: Vec<TypeEntry>,
    pub stats: Vec<StatEntry>,
    pub forms: Vec<NamedAPIResource>,
}

#[derive(Debug, Deserialize)]
pub struct TypeEntry {
    pub slot: i32,
    #[serde(rename = "type")]
    pub type_info: NamedAPIResource,
}

#[derive(Debug, Deserialize)]
pub struct StatEntry {
    pub base_stat: i32,
    pub stat: NamedAPIResource,
}

impl PokemonResponse {
    /// ドメインモデルに変換
    /// species_name_ja: 種族名の日本語（例: "キュウコン"）
    /// 日本語フォーム名は fullname と species_name_ja から自動生成される
    pub fn into_domain(self, species_id: i32, species_name_ja: Option<String>) -> PokemonForm {
        // タイプ情報を取得
        let mut type1 = String::new();
        let mut type2: Option<String> = None;

        for type_entry in &self.types {
            if type_entry.slot == 1 {
                type1 = type_entry.type_info.name.clone();
            } else if type_entry.slot == 2 {
                type2 = Some(type_entry.type_info.name.clone());
            }
        }

        // 種族値を取得
        let mut hp = 0;
        let mut attack = 0;
        let mut defense = 0;
        let mut sp_attack = 0;
        let mut sp_defense = 0;
        let mut speed = 0;

        for stat in &self.stats {
            match stat.stat.name.as_str() {
                "hp" => hp = stat.base_stat,
                "attack" => attack = stat.base_stat,
                "defense" => defense = stat.base_stat,
                "special-attack" => sp_attack = stat.base_stat,
                "special-defense" => sp_defense = stat.base_stat,
                "speed" => speed = stat.base_stat,
                _ => {}
            }
        }

        let base_stats = BaseStats::new(hp, attack, defense, sp_attack, sp_defense, speed);

        // フォーム名を抽出（例: "pikachu-alola" -> Some("alola")）
        let form_name = if self.name.contains('-') {
            Some(self.name.split('-').skip(1).collect::<Vec<_>>().join("-"))
        } else {
            None
        };

        // 日本語フォーム名を生成（例: "ninetales-alola" + "キュウコン" -> "キュウコン(アローラのすがた)"）
        let fullname_ja = localize_fullname_ja(&self.name, species_name_ja.as_deref());

        PokemonForm::new(
            self.id,
            species_id,
            form_name,
            self.name,
            fullname_ja,
            type1,
            type2,
            base_stats,
        )
    }

    /// このpokemonに属する全てのform URLを取得
    pub fn get_form_urls(&self) -> Vec<String> {
        self.forms.iter().map(|f| f.url.clone()).collect()
    }
}

/// PokeAPI pokemon-form レスポンス
#[derive(Debug, Deserialize)]
pub struct PokemonFormResponse {
    pub id: i32,
    pub name: String,
    pub form_name: String,
    pub types: Vec<TypeEntry>,
    pub names: Vec<FormNameEntry>,
}

#[derive(Debug, Deserialize)]
pub struct FormNameEntry {
    pub name: String,
    pub language: Language,
}

impl PokemonFormResponse {
    /// ドメインモデルに変換（親pokemonのstatsを使用）
    /// species_name_ja: 種族名の日本語（リージョン等のサフィックス付与に使用）
    pub fn into_domain(
        self,
        species_id: i32,
        base_stats: crate::domain::BaseStats,
        species_name_ja: Option<String>,
    ) -> PokemonForm {
        // タイプ情報を取得
        let mut type1 = String::new();
        let mut type2: Option<String> = None;

        for type_entry in &self.types {
            if type_entry.slot == 1 {
                type1 = type_entry.type_info.name.clone();
            } else if type_entry.slot == 2 {
                type2 = Some(type_entry.type_info.name.clone());
            }
        }

        // 日本語フォーム名を生成（ローカライズ関数を使用）
        let fullname_ja = localize_fullname_ja(&self.name, species_name_ja.as_deref());

        // フォーム名
        let form_name = if self.form_name.is_empty() {
            None
        } else {
            Some(self.form_name)
        };

        PokemonForm::new(
            self.id,
            species_id,
            form_name,
            self.name,
            fullname_ja,
            type1,
            type2,
            base_stats,
        )
    }
}

/// ポケモンリストアイテム
#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonListItem {
    pub name: String,
    pub url: String,
}

/// ポケモンリストレスポンス
#[derive(Debug, Deserialize)]
pub struct PokemonListResponse {
    #[allow(dead_code)]
    pub count: i32,
    #[allow(dead_code)]
    pub next: Option<String>,
    #[allow(dead_code)]
    pub previous: Option<String>,
    pub results: Vec<PokemonListItem>,
}
