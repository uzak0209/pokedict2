use serde::{Deserialize, Serialize};

/// チームエンティティ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    /// チームID
    pub team_id: i32,
    /// ユーザーID
    pub user_id: String,
    /// チーム名
    pub team_name: String,
    /// 説明（オプション）
    pub description: Option<String>,
    /// 作成日時
    pub created_at: Option<chrono::NaiveDateTime>,
    /// 更新日時
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Team {
    #[must_use]
    pub fn new(user_id: String, team_name: String, description: Option<String>) -> Self {
        Self {
            team_id: 0, // データベースで自動採番
            user_id,
            team_name,
            description,
            created_at: None,
            updated_at: None,
        }
    }
}
