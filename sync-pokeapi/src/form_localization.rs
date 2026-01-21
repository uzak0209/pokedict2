/// フォーム名の日本語ローカライズ
/// 英語のフォームサフィックスを日本語に変換し、種族名に追加する

/// フォームサフィックスを日本語に変換（汎用）
fn get_form_suffix_ja(form_suffix: &str) -> Option<&'static str> {
    match form_suffix {
        // リージョンフォーム
        "alola" | "alolan" => Some("(アローラのすがた)"),
        "galar" | "galarian" => Some("(ガラルのすがた)"),
        "hisui" | "hisuian" => Some("(ヒスイのすがた)"),
        "paldea" | "paldean" => Some("(パルデアのすがた)"),

        // メガシンカ
        "mega" => Some("(メガシンカ)"),
        "mega-x" => Some("(メガシンカX)"),
        "mega-y" => Some("(メガシンカY)"),

        // キョダイマックス
        "gmax" => Some("(キョダイマックス)"),

        // フォルムチェンジ系
        "origin" => Some("(オリジンフォルム)"),
        "altered" => Some("(アナザーフォルム)"),
        "sky" => Some("(スカイフォルム)"),
        "land" => Some("(ランドフォルム)"),
        "incarnate" => Some("(けしんフォルム)"),
        "therian" => Some("(れいじゅうフォルム)"),
        "attack" => Some("(アタックフォルム)"),
        "defense" => Some("(ディフェンスフォルム)"),
        "speed" => Some("(スピードフォルム)"),

        // バトル関連フォーム
        "blade" => Some("(ブレードフォルム)"),
        "shield" => Some("(シールドフォルム)"),

        // ネクロズマ
        "dusk-mane" => Some("(たそがれのたてがみ)"),
        "dawn-wings" => Some("(あかつきのつばさ)"),
        "dawn" => Some("(あかつきのつばさ)"),
        "ultra" => Some("(ウルトラネクロズマ)"),

        // ウーラオス
        "single-strike" => Some("(いちげきのかた)"),
        "rapid-strike" => Some("(れんげきのかた)"),

        // オーガポン
        "cornerstone" | "cornerstone-mask" => Some("(いしずえのめん)"),
        "hearthflame" | "hearthflame-mask" => Some("(かまどのめん)"),
        "wellspring" | "wellspring-mask" => Some("(いどのめん)"),

        // 性別フォーム
        "male" => Some("(オス)"),
        "female" => Some("(メス)"),

        // その他特殊フォーム
        "crowned" => Some("(けんのおう)"),
        "crowned-sword" => Some("(けんのおう)"),
        "crowned-shield" => Some("(たてのおう)"),
        "eternamax" => Some("(ムゲンダイマックス)"),
        "complete" => Some("(パーフェクトフォルム)"),
        "10" | "10-power-construct" => Some("(10%フォルム)"),
        "50" | "50-power-construct" => Some("(50%フォルム)"),

        // その他
        "disguised" => Some(""), // ミミッキュは普通そのまま
        "busted" => Some("(ばれたすがた)"),
        "antique" => Some("(しんさくフォルム)"),
        "phony" => Some("(がんさくフォルム)"),
        "unremarkable" => Some("(ぼんさいフォルム)"),

        // Palafin
        "zero" => Some(""),
        "hero" => Some("(ヒーローフォルム)"),

        // Tatsugiri
        "curly" | "droopy" | "stretchy" => Some(""),

        _ => None,
    }
}

/// 英語名（fullname）と日本語種族名から日本語フォーム名を生成
/// 例: ("ninetales-alola", "キュウコン") -> "キュウコン(アローラのすがた)"
/// 例: ("arceus-normal", "アルセウス") -> "アルセウス(ノーマル)"
pub fn localize_fullname_ja(fullname: &str, species_name_ja: Option<&str>) -> Option<String> {
    let species_ja = species_name_ja?;

    // フォームサフィックスを抽出
    let form_suffix = if fullname.contains('-') {
        fullname.split('-').skip(1).collect::<Vec<_>>().join("-")
    } else {
        // フォームサフィックスがない場合は種族名をそのまま返す
        return Some(species_ja.to_string());
    };

    // 特定のポケモンは専用処理（サフィックスが他と競合するため）
    let suffix_ja = match fullname {
        // Arceus/Silvally のタイプ別フォーム
        name if name.starts_with("arceus-") || name.starts_with("silvally-") => {
            get_type_suffix_ja(&form_suffix)
        }
        // Lycanroc はフォーム専用
        name if name.starts_with("lycanroc-") => match form_suffix.as_str() {
            "midday" => Some("(まひるのすがた)"),
            "midnight" => Some("(まよなかのすがた)"),
            "dusk" => Some("(たそがれのすがた)"),
            _ => None,
        },
        // Calyrex
        name if name.starts_with("calyrex-") => match form_suffix.as_str() {
            "ice" | "ice-rider" => Some("(はくばじょうのすがた)"),
            "shadow" | "shadow-rider" => Some("(こくばじょうのすがた)"),
            _ => None,
        },
        // Rotom
        name if name.starts_with("rotom-") => match form_suffix.as_str() {
            "heat" => Some("(ヒートロトム)"),
            "wash" => Some("(ウォッシュロトム)"),
            "frost" => Some("(フロストロトム)"),
            "fan" => Some("(スピンロトム)"),
            "mow" => Some("(カットロトム)"),
            _ => None,
        },
        // デフォルト
        _ => get_form_suffix_ja(&form_suffix),
    };

    if let Some(suffix) = suffix_ja {
        Some(format!("{}{}", species_ja, suffix))
    } else {
        // マッピングがない場合は種族名だけ返す
        Some(species_ja.to_string())
    }
}

/// タイプサフィックス専用（Arceus/Silvally）
fn get_type_suffix_ja(type_suffix: &str) -> Option<&'static str> {
    match type_suffix {
        "normal" => Some("(ノーマル)"),
        "bug" => Some("(むし)"),
        "dark" => Some("(あく)"),
        "dragon" => Some("(ドラゴン)"),
        "electric" => Some("(でんき)"),
        "fairy" => Some("(フェアリー)"),
        "fighting" => Some("(かくとう)"),
        "fire" => Some("(ほのお)"),
        "flying" => Some("(ひこう)"),
        "ghost" => Some("(ゴースト)"),
        "grass" => Some("(くさ)"),
        "ground" => Some("(じめん)"),
        "ice" => Some("(こおり)"),
        "poison" => Some("(どく)"),
        "psychic" => Some("(エスパー)"),
        "rock" => Some("(いわ)"),
        "steel" => Some("(はがね)"),
        "water" => Some("(みず)"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alola_form() {
        assert_eq!(
            localize_fullname_ja("ninetales-alola", Some("キュウコン")),
            Some("キュウコン(アローラのすがた)".to_string())
        );
    }

    #[test]
    fn test_arceus_type() {
        assert_eq!(
            localize_fullname_ja("arceus-normal", Some("アルセウス")),
            Some("アルセウス(ノーマル)".to_string())
        );
        assert_eq!(
            localize_fullname_ja("arceus-fire", Some("アルセウス")),
            Some("アルセウス(ほのお)".to_string())
        );
    }

    #[test]
    fn test_hisui_form() {
        assert_eq!(
            localize_fullname_ja("decidueye-hisui", Some("ジュナイパー")),
            Some("ジュナイパー(ヒスイのすがた)".to_string())
        );
    }

    #[test]
    fn test_base_form() {
        assert_eq!(
            localize_fullname_ja("bulbasaur", Some("フシギダネ")),
            Some("フシギダネ".to_string())
        );
    }
}
