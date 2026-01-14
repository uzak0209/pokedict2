use regex::Regex;

/// メールアドレスを表すvalue object
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailValidationError {
    Empty,
    InvalidFormat,
    TooLong,
}

impl Email {
    const MAX_LENGTH: usize = 254; // RFC 5321

    /// 新しいEmailインスタンスを作成
    ///
    /// # Errors
    ///
    /// - 空文字列の場合は `EmailValidationError::Empty`
    /// - メールアドレスの形式が不正な場合は `EmailValidationError::InvalidFormat`
    /// - 254文字を超える場合は `EmailValidationError::TooLong`
    pub fn new(email: &str) -> Result<Self, EmailValidationError> {
        let email = email.trim();

        if email.is_empty() {
            return Err(EmailValidationError::Empty);
        }

        if email.len() > Self::MAX_LENGTH {
            return Err(EmailValidationError::TooLong);
        }

        if !Self::is_valid_format(email) {
            return Err(EmailValidationError::InvalidFormat);
        }

        Ok(Self(email.to_lowercase()))
    }

    /// メールアドレスの形式をバリデーション
    #[allow(clippy::expect_used)]
    fn is_valid_format(email: &str) -> bool {
        // 基本的なメールアドレスのバリデーション
        // RFC 5322に完全準拠はしないが、一般的なケースをカバー
        // このregexは定数で安全なため、expectを使用
        let re = Regex::new(
            r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
        ).expect("Invalid regex pattern");

        re.is_match(email)
    }

    /// メールアドレスを文字列として取得
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::new("user@example.com");
        assert!(email.is_ok());
        assert_eq!(email.unwrap().as_str(), "user@example.com");
    }

    #[test]
    fn test_email_lowercase() {
        let email = Email::new("USER@EXAMPLE.COM").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_email_trimmed() {
        let email = Email::new("  user@example.com  ").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_empty_email() {
        let email = Email::new("");
        assert_eq!(email, Err(EmailValidationError::Empty));
    }

    #[test]
    fn test_invalid_format() {
        assert_eq!(
            Email::new("invalid"),
            Err(EmailValidationError::InvalidFormat)
        );
        assert_eq!(
            Email::new("@example.com"),
            Err(EmailValidationError::InvalidFormat)
        );
        assert_eq!(
            Email::new("user@"),
            Err(EmailValidationError::InvalidFormat)
        );
        assert_eq!(
            Email::new("user@@example.com"),
            Err(EmailValidationError::InvalidFormat)
        );
    }

    #[test]
    fn test_too_long_email() {
        let long_email = format!("{}@example.com", "a".repeat(250));
        assert_eq!(Email::new(&long_email), Err(EmailValidationError::TooLong));
    }

    #[test]
    fn test_valid_complex_emails() {
        assert!(Email::new("user.name@example.com").is_ok());
        assert!(Email::new("user+tag@example.co.jp").is_ok());
        assert!(Email::new("user_name@example-domain.com").is_ok());
    }
}
