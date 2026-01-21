use crate::domain::UserPokemon;
use crate::repository::pokemon_repository::RepositoryError;
use async_trait::async_trait;

/// UserPokemon リポジトリのトレイト
#[async_trait]
pub trait UserPokemonRepository: Send + Sync {
    /// ポケモンを作成（IDを返す）
    async fn create_pokemon(&self, pokemon: &UserPokemon) -> Result<i32, RepositoryError>;

    /// ポケモンIDでポケモンを取得
    async fn find_pokemon_by_id(&self, pokemon_id: i32)
        -> Result<Option<UserPokemon>, RepositoryError>;

    /// ユーザーIDで全ポケモンを取得
    async fn find_pokemon_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<UserPokemon>, RepositoryError>;

    /// ポケモンを更新
    async fn update_pokemon(&self, pokemon: &UserPokemon) -> Result<(), RepositoryError>;

    /// ポケモンを削除
    async fn delete_pokemon(&self, pokemon_id: i32) -> Result<(), RepositoryError>;
}
