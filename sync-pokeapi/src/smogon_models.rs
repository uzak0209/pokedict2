use serde::Deserialize;
use std::collections::HashMap;

/// Smogon Chaos JSON info セクション
#[derive(Debug, Deserialize)]
pub struct SmogonInfo {
    pub metagame: String,
    #[allow(dead_code)]
    pub cutoff: f64,
    #[serde(rename = "number of battles")]
    pub number_of_battles: i32,
}

/// Smogon ポケモンデータ
#[derive(Debug, Deserialize)]
pub struct SmogonPokemonData {
    #[serde(rename = "Raw count")]
    pub raw_count: i32,
    pub usage: f64,

    #[serde(rename = "Abilities", default)]
    pub abilities: HashMap<String, f64>,

    #[serde(rename = "Items", default)]
    pub items: HashMap<String, f64>,

    #[serde(rename = "Moves", default)]
    pub moves: HashMap<String, f64>,

    #[serde(rename = "Spreads", default)]
    pub spreads: HashMap<String, f64>,

    #[serde(rename = "Tera Types", default)]
    pub tera_types: HashMap<String, f64>,
}

/// Smogon Chaos JSON レスポンス
#[derive(Debug, Deserialize)]
pub struct SmogonChaosResponse {
    pub info: SmogonInfo,
    pub data: HashMap<String, SmogonPokemonData>,
}

impl SmogonChaosResponse {
    /// Smogon → PokeAPI 名前マッピング
    /// Smogonでは基本名のみ、PokeAPIでは特定のフォーム名が必要な場合のマッピング
    fn get_form_mapping(smogon_name: &str) -> Option<&'static str> {
        match smogon_name {
            // Genies (Incarnate forms are default in Smogon)
            "tornadus" => Some("tornadus-incarnate"),
            "thundurus" => Some("thundurus-incarnate"),
            "landorus" => Some("landorus-incarnate"),
            "enamorus" => Some("enamorus-incarnate"),

            // Urshifu
            "urshifu" => Some("urshifu-single-strike"),
            "urshifu-rapid-strike" => Some("urshifu-rapid-strike"),

            // Keldeo
            "keldeo" => Some("keldeo-ordinary"),

            // Necrozma
            "necrozma-dusk-mane" => Some("necrozma-dusk"),
            "necrozma-dawn-wings" => Some("necrozma-dawn"),

            // Giratina
            "giratina" => Some("giratina-altered"),

            // Ogerpon
            "ogerpon-cornerstone" => Some("ogerpon-cornerstone-mask"),
            "ogerpon-hearthflame" => Some("ogerpon-hearthflame-mask"),
            "ogerpon-wellspring" => Some("ogerpon-wellspring-mask"),

            // Palafin
            "palafin" => Some("palafin-zero"),

            // Gender forms
            "basculegion" => Some("basculegion-male"),
            "indeedee" => Some("indeedee-male"),
            "meowstic" => Some("meowstic-male"),
            "oinkologne" => Some("oinkologne-male"),

            // Lycanroc
            "lycanroc" => Some("lycanroc-midday"),

            // Special forms
            "mimikyu" => Some("mimikyu-disguised"),
            "morpeko" => Some("morpeko-full-belly"),
            "eiscue" => Some("eiscue-ice"),
            "minior" => Some("minior-red-meteor"),

            // Multiple patterns
            "basculin" => Some("basculin-red-striped"),
            "gastrodon" => Some("gastrodon-west"),
            "sawsbuck" => Some("sawsbuck-spring"),
            "deerling" => Some("deerling-spring"),
            "vivillon" => Some("vivillon-meadow"),
            "florges" => Some("florges-red"),
            "floette" => Some("floette-red"),
            "flabebe" => Some("flabebe-red"),

            // Alcremie
            "alcremie" => Some("alcremie-vanilla-cream-strawberry-sweet"),

            // Toxtricity
            "toxtricity" => Some("toxtricity-amped"),

            // Oricorio
            "oricorio" => Some("oricorio-baile"),

            // Dudunsparce
            "dudunsparce" => Some("dudunsparce-two-segment"),

            // Maushold
            "maushold" => Some("maushold-family-of-four"),

            // Squawkabilly
            "squawkabilly" => Some("squawkabilly-green-plumage"),

            // Tatsugiri
            "tatsugiri" => Some("tatsugiri-curly"),

            // Sinistcha/Polteageist
            "sinistcha" => Some("sinistcha-unremarkable"),
            "polteageist" => Some("polteageist-phony"),

            // Arceus (Smogon uses base name, DB has -normal)
            "arceus" => Some("arceus-normal"),

            // Meloetta
            "meloetta" => Some("meloetta-aria"),

            // Female suffix
            "basculegion-f" => Some("basculegion-female"),
            "indeedee-f" => Some("indeedee-female"),
            "oinkologne-f" => Some("oinkologne-female"),

            // Tauros Paldea
            "tauros-paldea-blaze" => Some("tauros-paldea-blaze-breed"),
            "tauros-paldea-aqua" => Some("tauros-paldea-aqua-breed"),
            "tauros-paldea-combat" => Some("tauros-paldea-combat-breed"),

            _ => None,
        }
    }

    /// Smogon名を正規化してfullnameにマッチさせる
    #[must_use]
    pub fn normalize_name(name: &str) -> String {
        let mut normalized = name.to_lowercase();

        // まずマッピングテーブルをチェック
        if let Some(mapped) = Self::get_form_mapping(&normalized) {
            return mapped.to_string();
        }

        // 特殊文字の前処理 (Mr. Mime, Type: Null)
        if normalized.starts_with("mr. ") {
            normalized = normalized.replace("mr. ", "mr-");
        }
        if normalized.starts_with("mime jr") {
            normalized = normalized.replace("mime jr", "mime-jr");
        }
        if normalized.starts_with("type: ") {
            normalized = normalized.replace("type: ", "type-");
        }

        // 一般的なスペース -> ハイフン変換
        // Paradox Pokemon (e.g. "Raging Bolt" -> "raging-bolt") もここで処理される
        normalized = normalized.replace(' ', "-");

        // アポストロフィ削除 ("Farfetch'd" -> "farfetchd")
        normalized = normalized.replace('\'', "");
        normalized = normalized.replace('’', ""); // スマートクォート対応

        // ピリオド削除
        normalized = normalized.replace('.', "");

        // 連続するハイフンを1つにまとめる
        while normalized.contains("--") {
            normalized = normalized.replace("--", "-");
        }

        normalized
    }
}
