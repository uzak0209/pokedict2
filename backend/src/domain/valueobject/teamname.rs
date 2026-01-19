#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeamNameValidationError {
    Empty,
    TooLong,
}

impl std::fmt::Display for TeamNameValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Team name cannot be empty"),
            Self::TooLong => write!(f, "Team name is too long (max 30 characters)"),
        }
    }
}

impl std::error::Error for TeamNameValidationError {}

impl TeamName {
    const MAX_LENGTH: usize = 30;

    /// 新しいチーム名を作成
    ///
    /// # Errors
    ///
    /// - 空文字列の場合は `TeamNameValidationError::Empty`
    /// - 30文字を超える場合は `TeamNameValidationError::TooLong`
    pub fn new(name: &str) -> Result<Self, TeamNameValidationError> {
        let name = name.trim();

        if name.is_empty() {
            return Err(TeamNameValidationError::Empty);
        }

        if name.chars().count() > Self::MAX_LENGTH {
            return Err(TeamNameValidationError::TooLong);
        }

        Ok(Self(name.to_string()))
    }

    /// チーム名を文字列として取得
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TeamName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
