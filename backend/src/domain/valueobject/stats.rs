use serde::{Deserialize, Serialize};

/// ステータス値（努力値・個体値用）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stats {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatsValidationError {
    EvTooHigh,      // 個別の努力値が252を超えている
    EvTotalTooHigh, // 努力値の合計が508を超えている
    IvTooHigh,      // 個体値が31を超えている
}

impl std::fmt::Display for StatsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EvTooHigh => write!(f, "EV value cannot exceed 252"),
            Self::EvTotalTooHigh => write!(f, "Total EV cannot exceed 508"),
            Self::IvTooHigh => write!(f, "IV value cannot exceed 31"),
        }
    }
}

impl std::error::Error for StatsValidationError {}

impl Stats {
    /// 努力値（EV）を作成
    ///
    /// # Errors
    ///
    /// - 個別の値が252を超える場合
    /// - 合計が508を超える場合
    pub fn new_ev(
        hp: u16,
        attack: u16,
        defense: u16,
        special_attack: u16,
        special_defense: u16,
        speed: u16,
    ) -> Result<Self, StatsValidationError> {
        // 個別チェック
        if hp > 252
            || attack > 252
            || defense > 252
            || special_attack > 252
            || special_defense > 252
            || speed > 252
        {
            return Err(StatsValidationError::EvTooHigh);
        }

        // 合計チェック
        let total = hp + attack + defense + special_attack + special_defense + speed;
        if total > 508 {
            return Err(StatsValidationError::EvTotalTooHigh);
        }

        Ok(Self {
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
        })
    }

    /// 個体値（IV）を作成
    ///
    /// # Errors
    ///
    /// - いずれかの値が31を超える場合
    pub fn new_iv(
        hp: u16,
        attack: u16,
        defense: u16,
        special_attack: u16,
        special_defense: u16,
        speed: u16,
    ) -> Result<Self, StatsValidationError> {
        if hp > 31
            || attack > 31
            || defense > 31
            || special_attack > 31
            || special_defense > 31
            || speed > 31
        {
            return Err(StatsValidationError::IvTooHigh);
        }

        Ok(Self {
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
        })
    }

    /// デフォルトの努力値（全て0）
    #[must_use]
    pub fn default_ev() -> Self {
        Self {
            hp: 0,
            attack: 0,
            defense: 0,
            special_attack: 0,
            special_defense: 0,
            speed: 0,
        }
    }

    /// デフォルトの個体値（全て31）
    #[must_use]
    pub fn default_iv() -> Self {
        Self {
            hp: 31,
            attack: 31,
            defense: 31,
            special_attack: 31,
            special_defense: 31,
            speed: 31,
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::default_ev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ev() {
        let ev = Stats::new_ev(252, 252, 4, 0, 0, 0);
        assert!(ev.is_ok());
    }

    #[test]
    fn test_ev_too_high() {
        let ev = Stats::new_ev(253, 0, 0, 0, 0, 0);
        assert_eq!(ev, Err(StatsValidationError::EvTooHigh));
    }

    #[test]
    fn test_ev_total_too_high() {
        let ev = Stats::new_ev(252, 252, 5, 0, 0, 0);
        assert_eq!(ev, Err(StatsValidationError::EvTotalTooHigh));
    }

    #[test]
    fn test_valid_iv() {
        let iv = Stats::new_iv(31, 31, 31, 31, 31, 31);
        assert!(iv.is_ok());
    }

    #[test]
    fn test_iv_too_high() {
        let iv = Stats::new_iv(32, 0, 0, 0, 0, 0);
        assert_eq!(iv, Err(StatsValidationError::IvTooHigh));
    }
}
