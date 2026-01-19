use std::sync::Arc;

use crate::domain::entity::user::User;
use crate::domain::valueobject::email::{Email, EmailValidationError};
use crate::domain::valueobject::hashedpassword::PasswordError;
use crate::domain::valueobject::username::{Username, UsernameValidationError};
use crate::repository::user_repository::{UserRepository, UserRepositoryError};

/// ユーザー登録のリクエスト
#[derive(Debug, Clone)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// ユーザー登録のレスポンス
#[derive(Debug, Clone)]
pub struct RegisterUserResponse {
    pub user_id: String,
    pub username: String,
    pub email: String,
}

/// ユーザー登録時のエラー
#[derive(Debug, thiserror::Error)]
pub enum RegisterUserError {
    #[error("Username validation failed: {0}")]
    UsernameValidation(String),
    #[error("Email validation failed: {0}")]
    EmailValidation(String),
    #[error("Password validation failed: {0}")]
    PasswordValidation(String),
    #[error("Username already exists")]
    UsernameAlreadyExists,
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("Repository error: {0}")]
    Repository(String),
}

impl From<UsernameValidationError> for RegisterUserError {
    fn from(err: UsernameValidationError) -> Self {
        match err {
            UsernameValidationError::TooShort => {
                Self::UsernameValidation("Username must be at least 3 characters".to_string())
            }
            UsernameValidationError::TooLong => {
                Self::UsernameValidation("Username must be at most 20 characters".to_string())
            }
            UsernameValidationError::InvalidCharacters => Self::UsernameValidation(
                "Username can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ),
        }
    }
}

impl From<EmailValidationError> for RegisterUserError {
    fn from(err: EmailValidationError) -> Self {
        match err {
            EmailValidationError::Empty => {
                Self::EmailValidation("Email cannot be empty".to_string())
            }
            EmailValidationError::InvalidFormat => {
                Self::EmailValidation("Invalid email format".to_string())
            }
            EmailValidationError::TooLong => Self::EmailValidation("Email is too long".to_string()),
        }
    }
}

impl From<PasswordError> for RegisterUserError {
    fn from(err: PasswordError) -> Self {
        match err {
            PasswordError::TooShort => {
                Self::PasswordValidation("Password must be at least 8 characters".to_string())
            }
            PasswordError::TooLong => Self::PasswordValidation("Password is too long".to_string()),
            PasswordError::HashingFailed => {
                Self::PasswordValidation("Failed to hash password".to_string())
            }
            PasswordError::InvalidHash => {
                Self::PasswordValidation("Invalid password hash".to_string())
            }
            PasswordError::VerificationFailed => {
                Self::PasswordValidation("Password verification failed".to_string())
            }
        }
    }
}

impl From<UserRepositoryError> for RegisterUserError {
    fn from(err: UserRepositoryError) -> Self {
        Self::Repository(err.to_string())
    }
}

/// ユーザー登録のユースケース
pub struct UserRegistrationUseCase<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UserRegistrationUseCase<R> {
    /// 新しいユースケースインスタンスを作成
    #[must_use]
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    /// ユーザーを登録
    ///
    /// # Errors
    ///
    /// - バリデーションエラーやリポジトリエラーが発生した場合
    pub async fn execute(
        &self,
        request: RegisterUserRequest,
    ) -> Result<RegisterUserResponse, RegisterUserError> {
        // 1. バリデーション
        let username = Username::new(&request.username)?;
        let email = Email::new(&request.email)?;

        // 2. 重複チェック
        if self.user_repository.exists_by_username(&username).await? {
            return Err(RegisterUserError::UsernameAlreadyExists);
        }

        if self.user_repository.exists_by_email(&email).await? {
            return Err(RegisterUserError::EmailAlreadyExists);
        }

        // 3. ユーザーエンティティの作成
        let user = User::new(username.clone(), email.clone(), &request.password)?;

        // 4. リポジトリに保存
        self.user_repository.save(&user).await?;

        // 5. レスポンスを返す
        Ok(RegisterUserResponse {
            user_id: user.user_id().to_string(),
            username: username.as_str().to_string(),
            email: email.as_str().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use uuid::Uuid;

    // テスト用のインメモリリポジトリ
    struct InMemoryUserRepository {
        users: Mutex<HashMap<Uuid, User>>,
    }

    impl InMemoryUserRepository {
        fn new() -> Self {
            Self {
                users: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl UserRepository for InMemoryUserRepository {
        async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
            let mut users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            users.insert(*user.user_id(), user.clone());
            Ok(())
        }

        async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, UserRepositoryError> {
            let users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            Ok(users.get(user_id).cloned())
        }

        async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError> {
            let users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            Ok(users.values().find(|u| u.email().as_str() == email.as_str()).cloned())
        }

        async fn find_by_username(
            &self,
            username: &Username,
        ) -> Result<Option<User>, UserRepositoryError> {
            let users = self
                .users
                .lock()
                .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
            Ok(users.values().find(|u| u.username().as_str() == username.as_str()).cloned())
        }

        async fn exists_by_email(&self, email: &Email) -> Result<bool, UserRepositoryError> {
            Ok(self.find_by_email(email).await?.is_some())
        }

        async fn exists_by_username(
            &self,
            username: &Username,
        ) -> Result<bool, UserRepositoryError> {
            Ok(self.find_by_username(username).await?.is_some())
        }
    }

    #[actix_rt::test]
    async fn test_register_user_success() {
        let repository = Arc::new(InMemoryUserRepository::new());
        let usecase = UserRegistrationUseCase::new(repository);

        let request = RegisterUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        let result = usecase.execute(request).await;
        assert!(result.is_ok());

        let response = result.ok().unwrap();
        assert_eq!(response.username, "testuser");
        assert_eq!(response.email, "test@example.com");
    }

    #[actix_rt::test]
    async fn test_register_user_duplicate_username() {
        let repository = Arc::new(InMemoryUserRepository::new());
        let usecase = UserRegistrationUseCase::new(repository);

        let request1 = RegisterUserRequest {
            username: "testuser".to_string(),
            email: "test1@example.com".to_string(),
            password: "password123".to_string(),
        };

        usecase.execute(request1).await.ok().unwrap();

        let request2 = RegisterUserRequest {
            username: "testuser".to_string(),
            email: "test2@example.com".to_string(),
            password: "password456".to_string(),
        };

        let result = usecase.execute(request2).await;
        assert!(matches!(
            result,
            Err(RegisterUserError::UsernameAlreadyExists)
        ));
    }

    #[actix_rt::test]
    async fn test_register_user_duplicate_email() {
        let repository = Arc::new(InMemoryUserRepository::new());
        let usecase = UserRegistrationUseCase::new(repository);

        let request1 = RegisterUserRequest {
            username: "testuser1".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        usecase.execute(request1).await.ok().unwrap();

        let request2 = RegisterUserRequest {
            username: "testuser2".to_string(),
            email: "test@example.com".to_string(),
            password: "password456".to_string(),
        };

        let result = usecase.execute(request2).await;
        assert!(matches!(result, Err(RegisterUserError::EmailAlreadyExists)));
    }
}
