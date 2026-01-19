use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// ポケモンの性格
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Nature {
    // 補正なし
    Hardy,
    Docile,
    Serious,
    Bashful,
    Quirky,

    // 攻撃↑
    Lonely,  // 防御↓
    Brave,   // 素早↓
    Adamant, // 特攻↓
    Naughty, // 特防↓

    // 防御↑
    Bold,   // 攻撃↓
    Relaxed, // 素早↓
    Impish, // 特攻↓
    Lax,    // 特防↓

    // 特攻↑
    Modest, // 攻撃↓
    Mild,   // 防御↓
    Quiet,  // 素早↓
    Rash,   // 特防↓

    // 特防↑
    Calm,    // 攻撃↓
    Gentle,  // 防御↓
    Sassy,   // 素早↓
    Careful, // 特攻↓

    // 素早↑
    Timid,  // 攻撃↓
    Hasty,  // 防御↓
    Jolly,  // 特攻↓
    Naive,  // 特防↓
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseNatureError;

impl std::fmt::Display for ParseNatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid nature")
    }
}

impl std::error::Error for ParseNatureError {}

impl FromStr for Nature {
    type Err = ParseNatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hardy" | "がんばりや" => Ok(Self::Hardy),
            "docile" | "すなお" => Ok(Self::Docile),
            "serious" | "まじめ" => Ok(Self::Serious),
            "bashful" | "てれや" => Ok(Self::Bashful),
            "quirky" | "きまぐれ" => Ok(Self::Quirky),
            "lonely" | "さみしがり" => Ok(Self::Lonely),
            "brave" | "ゆうかん" => Ok(Self::Brave),
            "adamant" | "いじっぱり" => Ok(Self::Adamant),
            "naughty" | "やんちゃ" => Ok(Self::Naughty),
            "bold" | "ずぶとい" => Ok(Self::Bold),
            "relaxed" | "のんき" => Ok(Self::Relaxed),
            "impish" | "わんぱく" => Ok(Self::Impish),
            "lax" | "のうてんき" => Ok(Self::Lax),
            "modest" | "ひかえめ" => Ok(Self::Modest),
            "mild" | "おっとり" => Ok(Self::Mild),
            "quiet" | "れいせい" => Ok(Self::Quiet),
            "rash" | "うっかりや" => Ok(Self::Rash),
            "calm" | "おだやか" => Ok(Self::Calm),
            "gentle" | "おとなしい" => Ok(Self::Gentle),
            "sassy" | "なまいき" => Ok(Self::Sassy),
            "careful" | "しんちょう" => Ok(Self::Careful),
            "timid" | "おくびょう" => Ok(Self::Timid),
            "hasty" | "せっかち" => Ok(Self::Hasty),
            "jolly" | "ようき" => Ok(Self::Jolly),
            "naive" | "むじゃき" => Ok(Self::Naive),
            _ => Err(ParseNatureError),
        }
    }
}

impl std::fmt::Display for Nature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Hardy => "Hardy",
            Self::Docile => "Docile",
            Self::Serious => "Serious",
            Self::Bashful => "Bashful",
            Self::Quirky => "Quirky",
            Self::Lonely => "Lonely",
            Self::Brave => "Brave",
            Self::Adamant => "Adamant",
            Self::Naughty => "Naughty",
            Self::Bold => "Bold",
            Self::Relaxed => "Relaxed",
            Self::Impish => "Impish",
            Self::Lax => "Lax",
            Self::Modest => "Modest",
            Self::Mild => "Mild",
            Self::Quiet => "Quiet",
            Self::Rash => "Rash",
            Self::Calm => "Calm",
            Self::Gentle => "Gentle",
            Self::Sassy => "Sassy",
            Self::Careful => "Careful",
            Self::Timid => "Timid",
            Self::Hasty => "Hasty",
            Self::Jolly => "Jolly",
            Self::Naive => "Naive",
        };
        write!(f, "{name}")
    }
}
