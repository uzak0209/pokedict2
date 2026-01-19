use serde::{Deserialize, Serialize};

/// ポケモンの技
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Move {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveValidationError {
    Empty,
    TooLong,
}

impl std::fmt::Display for MoveValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Move name cannot be empty"),
            Self::TooLong => write!(f, "Move name is too long"),
        }
    }
}

impl std::error::Error for MoveValidationError {}

impl Move {
    const MAX_LENGTH: usize = 50;

    /// 新しい技を作成
    ///
    /// # Errors
    ///
    /// - 空文字列の場合は `MoveValidationError::Empty`
    /// - 50文字を超える場合は `MoveValidationError::TooLong`
    pub fn new(name: &str) -> Result<Self, MoveValidationError> {
        let name = name.trim();

        if name.is_empty() {
            return Err(MoveValidationError::Empty);
        }

        if name.len() > Self::MAX_LENGTH {
            return Err(MoveValidationError::TooLong);
        }

        Ok(Self {
            name: name.to_string(),
        })
    }

    /// 技名を取得
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// 技のスロット（最大4つ）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveSet {
    moves: [Option<Move>; 4],
}

impl MoveSet {
    /// 空の技セットを作成
    #[must_use]
    pub fn empty() -> Self {
        Self {
            moves: [None, None, None, None],
        }
    }

    /// 技のリストから作成
    ///
    /// # Errors
    ///
    /// - 5つ以上の技を指定した場合
    pub fn from_vec(moves: Vec<Move>) -> Result<Self, MoveSetError> {
        if moves.len() > 4 {
            return Err(MoveSetError::TooManyMoves);
        }

        let mut move_array: [Option<Move>; 4] = [None, None, None, None];
        for (i, m) in moves.into_iter().enumerate() {
            move_array[i] = Some(m);
        }

        Ok(Self { moves: move_array })
    }

    /// 指定位置に技を設定
    ///
    /// # Errors
    ///
    /// - インデックスが0-3の範囲外の場合
    pub fn set_move(&mut self, index: usize, move_: Move) -> Result<(), MoveSetError> {
        if index >= 4 {
            return Err(MoveSetError::InvalidIndex);
        }
        self.moves[index] = Some(move_);
        Ok(())
    }

    /// 全技を取得
    #[must_use]
    pub fn moves(&self) -> &[Option<Move>; 4] {
        &self.moves
    }

    /// 登録されている技のみを取得
    #[must_use]
    pub fn move_list(&self) -> Vec<&Move> {
        self.moves.iter().filter_map(|m| m.as_ref()).collect()
    }

    /// 技の数を取得
    #[must_use]
    pub fn count(&self) -> usize {
        self.moves.iter().filter(|m| m.is_some()).count()
    }
}

impl Default for MoveSet {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MoveSetError {
    #[error("Too many moves (max 4)")]
    TooManyMoves,
    #[error("Invalid move index")]
    InvalidIndex,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_move() {
        let move_ = Move::new("Thunderbolt");
        assert!(move_.is_ok());
        assert_eq!(move_.unwrap().name(), "Thunderbolt");
    }

    #[test]
    fn test_move_empty() {
        let move_ = Move::new("");
        assert_eq!(move_, Err(MoveValidationError::Empty));
    }

    #[test]
    fn test_move_too_long() {
        let long_name = "a".repeat(51);
        let move_ = Move::new(&long_name);
        assert_eq!(move_, Err(MoveValidationError::TooLong));
    }

    #[test]
    fn test_moveset_from_vec() {
        let moves = vec![
            Move::new("Thunderbolt").unwrap(),
            Move::new("Ice Beam").unwrap(),
            Move::new("Surf").unwrap(),
            Move::new("Earthquake").unwrap(),
        ];

        let moveset = MoveSet::from_vec(moves);
        assert!(moveset.is_ok());
        assert_eq!(moveset.unwrap().count(), 4);
    }

    #[test]
    fn test_moveset_too_many() {
        let moves = vec![
            Move::new("Move1").unwrap(),
            Move::new("Move2").unwrap(),
            Move::new("Move3").unwrap(),
            Move::new("Move4").unwrap(),
            Move::new("Move5").unwrap(),
        ];

        let moveset = MoveSet::from_vec(moves);
        assert!(matches!(moveset, Err(MoveSetError::TooManyMoves)));
    }

    #[test]
    fn test_moveset_partial() {
        let moves = vec![Move::new("Thunderbolt").unwrap(), Move::new("Ice Beam").unwrap()];

        let moveset = MoveSet::from_vec(moves).unwrap();
        assert_eq!(moveset.count(), 2);
    }
}
