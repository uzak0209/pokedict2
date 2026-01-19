use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use crate::domain::entity::user::User;
use crate::domain::valueobject::email::Email;
use crate::domain::valueobject::username::Username;
use crate::repository::user_repository::{UserRepository, UserRepositoryError};

/// インメモリユーザーリポジトリ（開発・テスト用）
pub struct MockUserRepository {
    users: Mutex<HashMap<Uuid, User>>,
}

impl MockUserRepository {
    /// 新しいモックリポジトリを作成
    #[must_use]
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for MockUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
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
        Ok(users
            .values()
            .find(|u| u.email().as_str() == email.as_str())
            .cloned())
    }

    async fn find_by_username(
        &self,
        username: &Username,
    ) -> Result<Option<User>, UserRepositoryError> {
        let users = self
            .users
            .lock()
            .map_err(|e| UserRepositoryError::DatabaseError(format!("Lock error: {e}")))?;
        Ok(users
            .values()
            .find(|u| u.username().as_str() == username.as_str())
            .cloned())
    }

    async fn exists_by_email(&self, email: &Email) -> Result<bool, UserRepositoryError> {
        Ok(self.find_by_email(email).await?.is_some())
    }

    async fn exists_by_username(&self, username: &Username) -> Result<bool, UserRepositoryError> {
        Ok(self.find_by_username(username).await?.is_some())
    }
}
