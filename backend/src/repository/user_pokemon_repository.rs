use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entity::user_pokemon::UserPokemon;

/// ユーザーポケモンリポジトリのエラー
#[derive(Debug, thiserror::Error)]
pub enum UserPokemonRepositoryError {
    #[error("Pokemon not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// ユーザーポケモンリポジトリのトレイト
#[async_trait]
pub trait UserPokemonRepository: Send + Sync {
    /// ポケモンを保存
    ///
    /// # Errors
    ///
    /// データベースエラーが発生した場合
    async fn save(&self, pokemon: &UserPokemon) -> Result<(), UserPokemonRepositoryError>;

    /// IDでポケモンを検索
    ///
    /// # Errors
    ///
    /// データベースエラーが発生した場合
    async fn find_by_id(
        &self,
        pokemon_id: &Uuid,
    ) -> Result<Option<UserPokemon>, UserPokemonRepositoryError>;

    /// ユーザーIDで全ポケモンを検索
    ///
    /// # Errors
    ///
    /// データベースエラーが発生した場合
    async fn find_by_user_id(
        &self,
        user_id: &Uuid,
    ) -> Result<Vec<UserPokemon>, UserPokemonRepositoryError>;

    /// ポケモンを更新
    ///
    /// # Errors
    ///
    /// データベースエラーが発生した場合
    async fn update(&self, pokemon: &UserPokemon) -> Result<(), UserPokemonRepositoryError>;

    /// ポケモンを削除
    ///
    /// # Errors
    ///
    /// データベースエラーが発生した場合
    async fn delete(&self, pokemon_id: &Uuid) -> Result<(), UserPokemonRepositoryError>;
}
