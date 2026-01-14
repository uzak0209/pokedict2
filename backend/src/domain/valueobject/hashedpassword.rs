use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};

/// bcryptでハッシュ化されたパスワードを表すvalue object
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashedPassword(String);

#[derive(Debug, PartialEq, Eq)]
pub enum PasswordError {
    TooShort,
    TooLong,
    HashingFailed,
    InvalidHash,
    VerificationFailed,
}

impl From<BcryptError> for PasswordError {
    fn from(_: BcryptError) -> Self {
        Self::HashingFailed
    }
}

impl HashedPassword {
    const MIN_PASSWORD_LENGTH: usize = 8;
    const MAX_PASSWORD_LENGTH: usize = 72; // bcryptの制限

    /// 平文のパスワードからハッシュ化されたパスワードを生成
    ///
    /// # Errors
    ///
    /// - パスワードが8文字未満の場合は `PasswordError::TooShort`
    /// - パスワードが72文字を超える場合は `PasswordError::TooLong`
    /// - ハッシュ化に失敗した場合は `PasswordError::HashingFailed`
    ///
    /// # Examples
    ///
    /// ```
    /// use backend::domain::valueobject::hashedpassword::HashedPassword;
    ///
    /// let password = "my_secure_password123";
    /// let hashed = HashedPassword::from_plain(password).unwrap();
    /// assert!(hashed.verify(password).unwrap());
    /// ```
    pub fn from_plain(plain_password: &str) -> Result<Self, PasswordError> {
        if plain_password.len() < Self::MIN_PASSWORD_LENGTH {
            return Err(PasswordError::TooShort);
        }

        if plain_password.len() > Self::MAX_PASSWORD_LENGTH {
            return Err(PasswordError::TooLong);
        }

        let hashed = hash(plain_password, DEFAULT_COST)?;
        Ok(Self(hashed))
    }

    /// すでにハッシュ化された文字列から`HashedPassword`を作成
    /// 主にDBから取得した値を復元する際に使用
    ///
    /// # Errors
    ///
    /// - ハッシュの形式が不正な場合は `PasswordError::InvalidHash`
    pub fn from_hash(hash: &str) -> Result<Self, PasswordError> {
        // bcryptハッシュの基本的な検証（$2a$や$2b$で始まり、60文字程度）
        if !hash.starts_with("$2") || hash.len() < 59 {
            return Err(PasswordError::InvalidHash);
        }
        Ok(Self(hash.to_string()))
    }

    /// 平文のパスワードがこのハッシュと一致するか検証
    ///
    /// # Errors
    ///
    /// - 検証処理に失敗した場合は `PasswordError::VerificationFailed`
    pub fn verify(&self, plain_password: &str) -> Result<bool, PasswordError> {
        verify(plain_password, &self.0).map_err(|_| PasswordError::VerificationFailed)
    }

    /// ハッシュ化された文字列を取得（DB保存用）
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "my_secure_password123";
        let hashed = HashedPassword::from_plain(password).unwrap();

        // 正しいパスワードで検証
        assert!(hashed.verify(password).unwrap());

        // 間違ったパスワードで検証
        assert!(!hashed.verify("wrong_password").unwrap());
    }

    #[test]
    fn test_password_too_short() {
        let short_password = "short";
        let result = HashedPassword::from_plain(short_password);
        assert_eq!(result, Err(PasswordError::TooShort));
    }

    #[test]
    fn test_password_too_long() {
        let long_password = "a".repeat(73);
        let result = HashedPassword::from_plain(&long_password);
        assert_eq!(result, Err(PasswordError::TooLong));
    }

    #[test]
    fn test_minimum_length_password() {
        let password = "12345678"; // 8文字
        let result = HashedPassword::from_plain(password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_hash_valid() {
        // 実際のbcryptハッシュ
        let hash = "$2b$12$K3JxNVqQz4.FT5Y9Z6YQ5.m8kKkZGJtX7JqYXX5qRHzX7JqYXX5qR";
        let result = HashedPassword::from_hash(hash);
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_hash_invalid() {
        let invalid_hash = "not_a_valid_bcrypt_hash";
        let result = HashedPassword::from_hash(invalid_hash);
        assert_eq!(result, Err(PasswordError::InvalidHash));
    }

    #[test]
    fn test_different_passwords_different_hashes() {
        let password1 = "password123";
        let password2 = "password456";

        let hash1 = HashedPassword::from_plain(password1).unwrap();
        let hash2 = HashedPassword::from_plain(password2).unwrap();

        // 異なるパスワードは異なるハッシュになる
        assert_ne!(hash1.as_str(), hash2.as_str());
    }

    #[test]
    fn test_same_password_different_hashes() {
        let password = "password123";

        let hash1 = HashedPassword::from_plain(password).unwrap();
        let hash2 = HashedPassword::from_plain(password).unwrap();

        // bcryptはソルトを使うので、同じパスワードでも異なるハッシュになる
        assert_ne!(hash1.as_str(), hash2.as_str());

        // ただし、どちらも同じパスワードで検証できる
        assert!(hash1.verify(password).unwrap());
        assert!(hash2.verify(password).unwrap());
    }
}
