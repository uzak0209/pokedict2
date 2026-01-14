use uuid::Uuid;

use crate::domain::valueobject::email::Email;
use crate::domain::valueobject::hashedpassword::{HashedPassword, PasswordError};
use crate::domain::valueobject::username::Username;

/// ユーザーエンティティ
#[derive(Debug, Clone)]
#[allow(clippy::struct_field_names)]
pub struct User {
    user_id: Uuid,
    username: Username,
    email: Email,
    hashed_password: HashedPassword,
}

impl User {
    /// 新しいユーザーを作成（ユーザー登録時）
    ///
    /// # Errors
    ///
    /// - パスワードのハッシュ化に失敗した場合は `PasswordError` を返す
    pub fn new(
        username: Username,
        email: Email,
        plain_password: &str,
    ) -> Result<Self, PasswordError> {
        let hashed_password = HashedPassword::from_plain(plain_password)?;

        Ok(Self {
            user_id: Uuid::new_v4(),
            username,
            email,
            hashed_password,
        })
    }

    /// DBから復元する際に使用（すでにハッシュ化されたパスワードを持つ）
    ///
    /// # Errors
    ///
    /// - ハッシュの形式が不正な場合は `PasswordError` を返す
    pub fn from_repository(
        user_id: Uuid,
        username: Username,
        email: Email,
        password_hash: &str,
    ) -> Result<Self, PasswordError> {
        let hashed_password = HashedPassword::from_hash(password_hash)?;

        Ok(Self {
            user_id,
            username,
            email,
            hashed_password,
        })
    }

    /// パスワードを検証
    ///
    /// # Errors
    ///
    /// - 検証処理に失敗した場合は `PasswordError` を返す
    pub fn verify_password(&self, plain_password: &str) -> Result<bool, PasswordError> {
        self.hashed_password.verify(plain_password)
    }

    /// ユーザーIDを取得
    #[must_use]
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    /// ユーザー名を取得
    #[must_use]
    pub fn username(&self) -> &Username {
        &self.username
    }

    /// メールアドレスを取得
    #[must_use]
    pub fn email(&self) -> &Email {
        &self.email
    }

    /// ハッシュ化されたパスワードを取得（DB保存用）
    #[must_use]
    pub fn password_hash(&self) -> &str {
        self.hashed_password.as_str()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::domain::valueobject::email::Email;
    use crate::domain::valueobject::username::Username;

    #[test]
    fn test_create_new_user() {
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        let password = "secure_password123";

        let user = User::new(username, email, password).unwrap();

        assert!(user.verify_password(password).unwrap());
        assert!(!user.verify_password("wrong_password").unwrap());
    }

    #[test]
    fn test_user_from_repository() {
        let user_id = Uuid::new_v4();
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();

        // 実際のbcryptハッシュを生成
        let password = "secure_password123";
        let hashed = HashedPassword::from_plain(password).unwrap();

        let user = User::from_repository(user_id, username, email, hashed.as_str()).unwrap();

        assert_eq!(user.user_id(), &user_id);
        assert!(user.verify_password(password).unwrap());
    }

    #[test]
    fn test_password_verification_fails_with_wrong_password() {
        let username = Username::new("testuser").unwrap();
        let email = Email::new("test@example.com").unwrap();
        let password = "correct_password";

        let user = User::new(username, email, password).unwrap();

        assert!(!user.verify_password("wrong_password").unwrap());
    }
}
