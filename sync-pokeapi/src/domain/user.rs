use serde::{Deserialize, Serialize};

/// ユーザーエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// ユーザーID（UUID）
    pub user_id: String,
    /// ユーザー名
    pub username: String,
    /// メールアドレス（オプション）
    pub email: Option<String>,
    /// 作成日時
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl User {
    #[must_use]
    pub fn new(user_id: String, username: String, email: Option<String>) -> Self {
        Self {
            user_id,
            username,
            email,
            created_at: None,
        }
    }
}
