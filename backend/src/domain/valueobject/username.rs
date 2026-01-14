/// ユーザー名を表すvalue object
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Username(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsernameValidationError {
    TooShort,
    TooLong,
    InvalidCharacters,
}

impl Username {
    const MIN_LENGTH: usize = 3;
    const MAX_LENGTH: usize = 20;

    /// 新しいユーザー名を作成
    ///
    /// # Errors
    ///
    /// - 3文字未満の場合は `UsernameValidationError::TooShort`
    /// - 20文字を超える場合は `UsernameValidationError::TooLong`
    /// - 英数字、アンダースコア、ハイフン以外が含まれる場合は `UsernameValidationError::InvalidCharacters`
    pub fn new(name: &str) -> Result<Self, UsernameValidationError> {
        let name = name.trim();
        let count = name.chars().count();

        if count < Self::MIN_LENGTH {
            return Err(UsernameValidationError::TooShort);
        }

        if count > Self::MAX_LENGTH {
            return Err(UsernameValidationError::TooLong);
        }

        // 英数字、アンダースコア、ハイフンのみ許可
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            return Err(UsernameValidationError::InvalidCharacters);
        }

        Ok(Self(name.to_string()))
    }

    /// ユーザー名を文字列として取得
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_username() {
        assert!(Username::new("user123").is_ok());
        assert!(Username::new("test_user").is_ok());
        assert!(Username::new("user-name").is_ok());
        assert!(Username::new("abc").is_ok());
    }

    #[test]
    fn test_username_too_short() {
        assert_eq!(Username::new("ab"), Err(UsernameValidationError::TooShort));
    }

    #[test]
    fn test_username_too_long() {
        let long_name = "a".repeat(21);
        assert_eq!(
            Username::new(&long_name),
            Err(UsernameValidationError::TooLong)
        );
    }

    #[test]
    fn test_invalid_characters() {
        assert_eq!(
            Username::new("user@name"),
            Err(UsernameValidationError::InvalidCharacters)
        );
        assert_eq!(
            Username::new("user name"),
            Err(UsernameValidationError::InvalidCharacters)
        );
        assert_eq!(
            Username::new("user!"),
            Err(UsernameValidationError::InvalidCharacters)
        );
    }

    #[test]
    fn test_username_trimmed() {
        let username = Username::new("  testuser  ").unwrap();
        assert_eq!(username.as_str(), "testuser");
    }
}
