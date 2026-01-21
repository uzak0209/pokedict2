use crate::domain::entity::pokemon_form::PokemonForm;
use crate::domain::valueobject::teamname::TeamName;
use uuid::Uuid;

/// チームエンティティ
#[derive(Debug, Clone)]
#[allow(clippy::struct_field_names)]
pub struct Team {
    team_id: Uuid,
    owner_id: Uuid,
    team_name: TeamName,
    pokemon: [Option<PokemonForm>; 6],
}

impl Team {
    /// 新しいチームを作成
    #[must_use]
    pub fn new(owner_id: Uuid, team_name: TeamName) -> Self {
        Self {
            team_id: Uuid::new_v4(),
            owner_id,
            team_name,
            pokemon: [None, None, None, None, None, None],
        }
    }

    /// DBから復元する際に使用
    ///
    /// # Errors
    ///
    /// - ポケモンが7体以上の場合はエラー
    pub fn from_repository(
        team_id: Uuid,
        owner_id: Uuid,
        team_name: TeamName,
        pokemon: Vec<Option<PokemonForm>>,
    ) -> Result<Self, TeamError> {
        if pokemon.len() > 6 {
            return Err(TeamError::TooManyPokemon);
        }

        let mut pokemon_array: [Option<PokemonForm>; 6] = [None, None, None, None, None, None];
        for (i, p) in pokemon.into_iter().enumerate() {
            pokemon_array[i] = p;
        }

        Ok(Self {
            team_id,
            owner_id,
            team_name,
            pokemon: pokemon_array,
        })
    }

    /// チームIDを取得
    #[must_use]
    pub fn team_id(&self) -> &Uuid {
        &self.team_id
    }

    /// オーナーIDを取得
    #[must_use]
    pub fn owner_id(&self) -> &Uuid {
        &self.owner_id
    }

    /// チーム名を取得
    #[must_use]
    pub fn team_name(&self) -> &TeamName {
        &self.team_name
    }

    /// ポケモン配列を取得
    #[must_use]
    pub fn pokemon(&self) -> &[Option<PokemonForm>; 6] {
        &self.pokemon
    }

    /// 登録されているポケモンのみを取得
    #[must_use]
    pub fn pokemon_list(&self) -> Vec<&PokemonForm> {
        self.pokemon.iter().filter_map(|p| p.as_ref()).collect()
    }

    /// チーム名を更新
    pub fn update_name(&mut self, new_name: TeamName) {
        self.team_name = new_name;
    }

    /// 指定位置にポケモンを設定
    ///
    /// # Errors
    ///
    /// - インデックスが0-5の範囲外の場合はエラー
    pub fn set_pokemon(&mut self, index: usize, pokemon: PokemonForm) -> Result<(), TeamError> {
        if index >= 6 {
            return Err(TeamError::InvalidIndex);
        }
        self.pokemon[index] = Some(pokemon);
        Ok(())
    }

    /// 指定位置のポケモンを削除
    ///
    /// # Errors
    ///
    /// - インデックスが0-5の範囲外の場合はエラー
    pub fn remove_pokemon(&mut self, index: usize) -> Result<(), TeamError> {
        if index >= 6 {
            return Err(TeamError::InvalidIndex);
        }
        self.pokemon[index] = None;
        Ok(())
    }

    /// 空いている最初のスロットにポケモンを追加
    ///
    /// # Errors
    ///
    /// - チームが既に6体のポケモンで満たされている場合はエラー
    pub fn add_pokemon(&mut self, pokemon: PokemonForm) -> Result<(), TeamError> {
        for slot in &mut self.pokemon {
            if slot.is_none() {
                *slot = Some(pokemon);
                return Ok(());
            }
        }
        Err(TeamError::TeamFull)
    }

    /// ポケモンリストを更新（既存のポケモンを全て上書き）
    ///
    /// # Errors
    ///
    /// - 7体以上のポケモンを設定しようとした場合はエラー
    pub fn update_pokemon(&mut self, pokemon: Vec<PokemonForm>) -> Result<(), TeamError> {
        if pokemon.len() > 6 {
            return Err(TeamError::TooManyPokemon);
        }

        // 全スロットをクリア
        self.pokemon = [None, None, None, None, None, None];

        // 新しいポケモンを設定
        for (i, p) in pokemon.into_iter().enumerate() {
            self.pokemon[i] = Some(p);
        }

        Ok(())
    }

    /// チームに含まれるポケモンの数を取得
    #[must_use]
    pub fn pokemon_count(&self) -> usize {
        self.pokemon.iter().filter(|p| p.is_some()).count()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TeamError {
    #[error("Team is already full (max 6 Pokemon)")]
    TeamFull,
    #[error("Invalid Pokemon index")]
    InvalidIndex,
    #[error("Too many Pokemon (max 6)")]
    TooManyPokemon,
}
