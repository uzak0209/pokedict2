use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Smogon Usage Stats エンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    /// フォーマット名（例: "gen9bssregh"）
    pub format: String,
    /// 期間（例: "2024-12"）
    pub period: String,
    /// Pokemon form_id
    pub form_id: i32,
    /// 生カウント
    pub raw_count: i32,
    /// 使用率 (0.0-1.0)
    pub usage: f64,
    /// 特性データ（特性名 -> 使用回数）
    pub abilities: HashMap<String, f64>,
    /// 持ち物データ（持ち物名 -> 使用回数）
    pub items: HashMap<String, f64>,
    /// 技データ（技名 -> 使用回数）
    pub moves: HashMap<String, f64>,
    /// EVスプレッドデータ（"性格:HP/攻/防/特攻/特防/素早" -> 使用回数）
    pub spreads: HashMap<String, f64>,
    /// テラスタイプデータ（タイプ名 -> 使用回数）
    pub tera_types: HashMap<String, f64>,
}

impl UsageStats {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        format: String,
        period: String,
        form_id: i32,
        raw_count: i32,
        usage: f64,
        abilities: HashMap<String, f64>,
        items: HashMap<String, f64>,
        moves: HashMap<String, f64>,
        spreads: HashMap<String, f64>,
        tera_types: HashMap<String, f64>,
    ) -> Self {
        Self {
            format,
            period,
            form_id,
            raw_count,
            usage,
            abilities,
            items,
            moves,
            spreads,
            tera_types,
        }
    }
}
