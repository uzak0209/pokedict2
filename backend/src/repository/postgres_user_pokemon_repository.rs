use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::domain::entity::user_pokemon::UserPokemon;
use crate::domain::valueobject::move_slot::{Move, MoveSet};
use crate::domain::valueobject::nature::Nature;
use crate::domain::valueobject::pokemontype::PokemonType;
use crate::domain::valueobject::stats::Stats;
use crate::domain::valueobject::typeset::TypeSet;
use crate::repository::user_pokemon_repository::{UserPokemonRepository, UserPokemonRepositoryError};

/// データベースから取得した行の構造体
#[derive(FromRow)]
struct PokemonRow {
    pokemon_id: Uuid,
    user_id: Uuid,
    form_id: i32,
    nickname: Option<String>,
    nature: String,
    ability: String,
    item: Option<String>,
    tera_type: String,
    move1: String,
    move2: Option<String>,
    move3: Option<String>,
    move4: Option<String>,
    ev_hp: i32,
    ev_attack: i32,
    ev_defense: i32,
    ev_sp_attack: i32,
    ev_sp_defense: i32,
    ev_speed: i32,
    iv_hp: i32,
    iv_attack: i32,
    iv_defense: i32,
    iv_sp_attack: i32,
    iv_sp_defense: i32,
    iv_speed: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// PostgreSQLユーザーポケモンリポジトリ
pub struct PostgresUserPokemonRepository {
    pool: PgPool,
}

impl PostgresUserPokemonRepository {
    /// 新しいPostgreSQLリポジトリを作成
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// データベースマイグレーションを実行
    ///
    /// # Errors
    ///
    /// - マイグレーション実行に失敗した場合
    pub async fn migrate(&self) -> Result<(), UserPokemonRepositoryError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_pokemon (
                pokemon_id UUID PRIMARY KEY,
                user_id UUID NOT NULL,
                form_id INTEGER NOT NULL,
                nickname VARCHAR(100),
                nature VARCHAR(20) NOT NULL,
                ability VARCHAR(100) NOT NULL,
                item VARCHAR(100),
                tera_type VARCHAR(20) NOT NULL,
                move1 VARCHAR(100) NOT NULL,
                move2 VARCHAR(100),
                move3 VARCHAR(100),
                move4 VARCHAR(100),
                ev_hp INTEGER NOT NULL DEFAULT 0 CHECK (ev_hp >= 0 AND ev_hp <= 252),
                ev_attack INTEGER NOT NULL DEFAULT 0 CHECK (ev_attack >= 0 AND ev_attack <= 252),
                ev_defense INTEGER NOT NULL DEFAULT 0 CHECK (ev_defense >= 0 AND ev_defense <= 252),
                ev_sp_attack INTEGER NOT NULL DEFAULT 0 CHECK (ev_sp_attack >= 0 AND ev_sp_attack <= 252),
                ev_sp_defense INTEGER NOT NULL DEFAULT 0 CHECK (ev_sp_defense >= 0 AND ev_sp_defense <= 252),
                ev_speed INTEGER NOT NULL DEFAULT 0 CHECK (ev_speed >= 0 AND ev_speed <= 252),
                iv_hp INTEGER NOT NULL DEFAULT 31 CHECK (iv_hp >= 0 AND iv_hp <= 31),
                iv_attack INTEGER NOT NULL DEFAULT 31 CHECK (iv_attack >= 0 AND iv_attack <= 31),
                iv_defense INTEGER NOT NULL DEFAULT 31 CHECK (iv_defense >= 0 AND iv_defense <= 31),
                iv_sp_attack INTEGER NOT NULL DEFAULT 31 CHECK (iv_sp_attack >= 0 AND iv_sp_attack <= 31),
                iv_sp_defense INTEGER NOT NULL DEFAULT 31 CHECK (iv_sp_defense >= 0 AND iv_sp_defense <= 31),
                iv_speed INTEGER NOT NULL DEFAULT 31 CHECK (iv_speed >= 0 AND iv_speed <= 31),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| UserPokemonRepositoryError::DatabaseError(e.to_string()))?;

        // インデックスを作成
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_user_pokemon_user_id ON user_pokemon(user_id)
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| UserPokemonRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl UserPokemonRepository for PostgresUserPokemonRepository {
    async fn save(&self, pokemon: &UserPokemon) -> Result<(), UserPokemonRepositoryError> {
        // MoveSetから個別の技を取り出す
        let moves = pokemon.moves().moves();
        let move1 = moves[0].as_ref().map(Move::name).unwrap_or("");
        let move2 = moves[1].as_ref().map(Move::name);
        let move3 = moves[2].as_ref().map(Move::name);
        let move4 = moves[3].as_ref().map(Move::name);

        sqlx::query(
            r#"
            INSERT INTO user_pokemon (
                pokemon_id, user_id, form_id, nickname, nature, ability, item, tera_type,
                move1, move2, move3, move4,
                ev_hp, ev_attack, ev_defense, ev_sp_attack, ev_sp_defense, ev_speed,
                iv_hp, iv_attack, iv_defense, iv_sp_attack, iv_sp_defense, iv_speed,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25)
            ON CONFLICT (pokemon_id) DO UPDATE
            SET nickname = $4, nature = $5, ability = $6, item = $7, tera_type = $8,
                move1 = $9, move2 = $10, move3 = $11, move4 = $12,
                ev_hp = $13, ev_attack = $14, ev_defense = $15, ev_sp_attack = $16, ev_sp_defense = $17, ev_speed = $18,
                iv_hp = $19, iv_attack = $20, iv_defense = $21, iv_sp_attack = $22, iv_sp_defense = $23, iv_speed = $24
            "#,
        )
        .bind(pokemon.pokemon_id())
        .bind(pokemon.user_id())
        .bind(pokemon.form_id())
        .bind(pokemon.nickname())
        .bind(pokemon.nature().to_string())
        .bind(pokemon.ability())
        .bind(pokemon.held_item())
        .bind(pokemon.terastal_type().to_string())
        .bind(move1)
        .bind(move2)
        .bind(move3)
        .bind(move4)
        .bind(pokemon.ev().hp as i32)
        .bind(pokemon.ev().attack as i32)
        .bind(pokemon.ev().defense as i32)
        .bind(pokemon.ev().special_attack as i32)
        .bind(pokemon.ev().special_defense as i32)
        .bind(pokemon.ev().speed as i32)
        .bind(pokemon.iv().hp as i32)
        .bind(pokemon.iv().attack as i32)
        .bind(pokemon.iv().defense as i32)
        .bind(pokemon.iv().special_attack as i32)
        .bind(pokemon.iv().special_defense as i32)
        .bind(pokemon.iv().speed as i32)
        .bind(pokemon.created_at())
        .execute(&self.pool)
        .await
        .map_err(|e| UserPokemonRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, pokemon_id: &Uuid) -> Result<Option<UserPokemon>, UserPokemonRepositoryError> {
        let row = sqlx::query_as::<_, PokemonRow>(
            r#"
            SELECT
                pokemon_id, user_id, form_id, nickname, nature, ability, item, tera_type,
                move1, move2, move3, move4,
                ev_hp, ev_attack, ev_defense, ev_sp_attack, ev_sp_defense, ev_speed,
                iv_hp, iv_attack, iv_defense, iv_sp_attack, iv_sp_defense, iv_speed,
                created_at
            FROM user_pokemon
            WHERE pokemon_id = $1
            "#,
        )
        .bind(pokemon_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserPokemonRepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let pokemon = self.row_to_pokemon(row)?;
                Ok(Some(pokemon))
            }
            None => Ok(None),
        }
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<UserPokemon>, UserPokemonRepositoryError> {
        let rows = sqlx::query_as::<_, PokemonRow>(
            r#"
            SELECT
                pokemon_id, user_id, form_id, nickname, nature, ability, item, tera_type,
                move1, move2, move3, move4,
                ev_hp, ev_attack, ev_defense, ev_sp_attack, ev_sp_defense, ev_speed,
                iv_hp, iv_attack, iv_defense, iv_sp_attack, iv_sp_defense, iv_speed,
                created_at
            FROM user_pokemon
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| UserPokemonRepositoryError::DatabaseError(e.to_string()))?;

        let mut pokemon_list = Vec::new();
        for row in rows {
            let pokemon = self.row_to_pokemon(row)?;
            pokemon_list.push(pokemon);
        }

        Ok(pokemon_list)
    }

    async fn update(&self, pokemon: &UserPokemon) -> Result<(), UserPokemonRepositoryError> {
        // saveと同じロジックを使用（ON CONFLICT DO UPDATEで実装済み）
        self.save(pokemon).await
    }

    async fn delete(&self, pokemon_id: &Uuid) -> Result<(), UserPokemonRepositoryError> {
        sqlx::query(
            r#"
            DELETE FROM user_pokemon
            WHERE pokemon_id = $1
            "#,
        )
        .bind(pokemon_id)
        .execute(&self.pool)
        .await
        .map_err(|e| UserPokemonRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

impl PostgresUserPokemonRepository {
    /// データベース行をUserPokemonエンティティに変換
    fn row_to_pokemon(
        &self,
        row: PokemonRow,
    ) -> Result<UserPokemon, UserPokemonRepositoryError> {
        let PokemonRow {
            pokemon_id, user_id, form_id, nickname, nature, ability, item, tera_type,
            move1, move2, move3, move4,
            ev_hp, ev_attack, ev_defense, ev_sp_attack, ev_sp_defense, ev_speed,
            iv_hp, iv_attack, iv_defense, iv_sp_attack, iv_sp_defense, iv_speed,
            created_at,
        } = row;
        // Natureのパース
        let nature = nature.parse::<Nature>()
            .map_err(|_| UserPokemonRepositoryError::DatabaseError(format!("Invalid nature: {nature}")))?;

        // PokemonTypeのパース
        let tera_type = tera_type.parse::<PokemonType>()
            .map_err(|_| UserPokemonRepositoryError::DatabaseError(format!("Invalid tera type: {tera_type}")))?;

        // EVの作成
        let ev = Stats::new_ev(
            ev_hp as u16, ev_attack as u16, ev_defense as u16,
            ev_sp_attack as u16, ev_sp_defense as u16, ev_speed as u16,
        ).map_err(|e| UserPokemonRepositoryError::DatabaseError(format!("Invalid EV: {e}")))?;

        // IVの作成
        let iv = Stats::new_iv(
            iv_hp as u16, iv_attack as u16, iv_defense as u16,
            iv_sp_attack as u16, iv_sp_defense as u16, iv_speed as u16,
        ).map_err(|e| UserPokemonRepositoryError::DatabaseError(format!("Invalid IV: {e}")))?;

        // 技の変換
        let mut moves_vec = vec![Move::new(&move1)
            .map_err(|e| UserPokemonRepositoryError::DatabaseError(format!("Invalid move: {e}")))?];

        if let Some(m2) = move2 {
            moves_vec.push(Move::new(&m2)
                .map_err(|e| UserPokemonRepositoryError::DatabaseError(format!("Invalid move: {e}")))?);
        }
        if let Some(m3) = move3 {
            moves_vec.push(Move::new(&m3)
                .map_err(|e| UserPokemonRepositoryError::DatabaseError(format!("Invalid move: {e}")))?);
        }
        if let Some(m4) = move4 {
            moves_vec.push(Move::new(&m4)
                .map_err(|e| UserPokemonRepositoryError::DatabaseError(format!("Invalid move: {e}")))?);
        }

        let moves = MoveSet::from_vec(moves_vec)
            .map_err(|e| UserPokemonRepositoryError::DatabaseError(format!("Invalid move set: {e}")))?;

        // TODO: マスタデータから取得（現在は暫定的な値）
        let fullname = format!("Pokemon-{form_id}");
        let fullname_jp = format!("ポケモン-{form_id}");
        let species_id = form_id;
        let typeset = TypeSet::new(PokemonType::Normal, None);

        Ok(UserPokemon::from_repository(
            pokemon_id,
            user_id,
            nickname,
            form_id,
            species_id,
            fullname,
            fullname_jp,
            typeset,
            tera_type,
            ev,
            iv,
            nature,
            ability,
            item,
            moves,
            created_at,
        ))
    }
}
