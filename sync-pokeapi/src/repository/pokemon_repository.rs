use crate::domain::{PokemonForm, PokemonSpecies, UsageStats};
use async_trait::async_trait;
use thiserror::Error;

/// リポジトリエラー型
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Not found")]
    #[allow(dead_code)]
    NotFound,
}

/// Pokemon リポジトリのトレイト
#[async_trait]
pub trait PokemonRepository: Send + Sync {
    /// Species を保存
    async fn save_species(&self, species: &PokemonSpecies) -> Result<(), RepositoryError>;

    /// Form を保存
    async fn save_form(&self, form: &PokemonForm) -> Result<(), RepositoryError>;

    /// Species が存在するか確認
    async fn species_exists(&self, species_id: i32) -> Result<bool, RepositoryError>;

    /// Form が存在するか確認
    #[allow(dead_code)]
    async fn form_exists(&self, form_id: i32) -> Result<bool, RepositoryError>;

    /// Fullname から form_id を検索
    async fn find_form_id_by_fullname(
        &self,
        fullname: &str,
    ) -> Result<Option<i32>, RepositoryError>;

    /// Usage Stats を保存
    async fn save_usage_stats(&self, stats: &UsageStats) -> Result<(), RepositoryError>;

    /// 指定されたフォーマットと期間の Usage Stats を削除
    async fn delete_usage_stats(&self, format: &str, period: &str) -> Result<(), RepositoryError>;

    /// データベースマイグレーション（テーブル作成）
    async fn migrate(&self) -> Result<(), RepositoryError>;
}
