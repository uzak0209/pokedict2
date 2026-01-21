use crate::domain::{PokemonForm, PokemonSpecies, UsageStats};
use crate::repository::pokemon_repository::{PokemonRepository, RepositoryError};
use async_trait::async_trait;
use sqlx::PgPool;

/// PostgreSQL リポジトリ実装
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PokemonRepository for PostgresRepository {
    async fn save_species(&self, species: &PokemonSpecies) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO pokemon_species (species_id, name, name_ja, is_legendary, is_mythical)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (species_id) DO UPDATE SET
                name = EXCLUDED.name,
                name_ja = EXCLUDED.name_ja,
                is_legendary = EXCLUDED.is_legendary,
                is_mythical = EXCLUDED.is_mythical,
                updated_at = NOW()
            "#,
        )
        .bind(species.species_id)
        .bind(&species.name)
        .bind(&species.name_ja)
        .bind(species.is_legendary)
        .bind(species.is_mythical)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn save_form(&self, form: &PokemonForm) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO pokemon_forms (
                form_id, species_id, form_name, fullname, fullname_ja,
                type1, type2, hp, attack, defense, sp_attack, sp_defense, speed
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (form_id) DO UPDATE SET
                species_id = EXCLUDED.species_id,
                form_name = EXCLUDED.form_name,
                fullname = EXCLUDED.fullname,
                fullname_ja = EXCLUDED.fullname_ja,
                type1 = EXCLUDED.type1,
                type2 = EXCLUDED.type2,
                hp = EXCLUDED.hp,
                attack = EXCLUDED.attack,
                defense = EXCLUDED.defense,
                sp_attack = EXCLUDED.sp_attack,
                sp_defense = EXCLUDED.sp_defense,
                speed = EXCLUDED.speed,
                updated_at = NOW()
            "#,
        )
        .bind(form.form_id)
        .bind(form.species_id)
        .bind(&form.form_name)
        .bind(&form.fullname)
        .bind(&form.fullname_ja)
        .bind(&form.type1)
        .bind(&form.type2)
        .bind(form.base_stats.hp)
        .bind(form.base_stats.attack)
        .bind(form.base_stats.defense)
        .bind(form.base_stats.sp_attack)
        .bind(form.base_stats.sp_defense)
        .bind(form.base_stats.speed)
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn species_exists(&self, species_id: i32) -> Result<bool, RepositoryError> {
        let result: (bool,) =
            sqlx::query_as("SELECT EXISTS(SELECT 1 FROM pokemon_species WHERE species_id = $1)")
                .bind(species_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.0)
    }

    async fn form_exists(&self, form_id: i32) -> Result<bool, RepositoryError> {
        let result: (bool,) =
            sqlx::query_as("SELECT EXISTS(SELECT 1 FROM pokemon_forms WHERE form_id = $1)")
                .bind(form_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.0)
    }

    async fn find_form_id_by_fullname(
        &self,
        fullname: &str,
    ) -> Result<Option<i32>, RepositoryError> {
        let result: Option<(i32,)> =
            sqlx::query_as("SELECT form_id FROM pokemon_forms WHERE fullname = $1")
                .bind(fullname)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(result.map(|r| r.0))
    }

    async fn save_usage_stats(&self, stats: &UsageStats) -> Result<(), RepositoryError> {
        // Start a transaction
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Insert/Update main usage_stats record (form_id is primary key)
        sqlx::query(
            r#"
            INSERT INTO usage_stats (form_id, format, period, raw_count, usage, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            ON CONFLICT (form_id) DO UPDATE SET
                format = EXCLUDED.format,
                period = EXCLUDED.period,
                raw_count = EXCLUDED.raw_count,
                usage = EXCLUDED.usage,
                updated_at = NOW()
            "#,
        )
        .bind(stats.form_id)
        .bind(&stats.format)
        .bind(&stats.period)
        .bind(stats.raw_count)
        .bind(stats.usage)
        .execute(&mut *tx)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Delete existing related records (CASCADE will handle deletion automatically)
        sqlx::query("DELETE FROM usage_abilities WHERE form_id = $1")
            .bind(stats.form_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        sqlx::query("DELETE FROM usage_items WHERE form_id = $1")
            .bind(stats.form_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        sqlx::query("DELETE FROM usage_moves WHERE form_id = $1")
            .bind(stats.form_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        sqlx::query("DELETE FROM usage_spreads WHERE form_id = $1")
            .bind(stats.form_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        sqlx::query("DELETE FROM usage_tera_types WHERE form_id = $1")
            .bind(stats.form_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Insert abilities
        for (ability_name, percentage) in &stats.abilities {
            sqlx::query(
                r#"
                INSERT INTO usage_abilities (form_id, ability_name, percentage)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(stats.form_id)
            .bind(ability_name)
            .bind(percentage)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        // Insert items
        for (item_name, percentage) in &stats.items {
            sqlx::query(
                r#"
                INSERT INTO usage_items (form_id, item_name, percentage)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(stats.form_id)
            .bind(item_name)
            .bind(percentage)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        // Insert moves
        for (move_name, percentage) in &stats.moves {
            sqlx::query(
                r#"
                INSERT INTO usage_moves (form_id, move_name, percentage)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(stats.form_id)
            .bind(move_name)
            .bind(percentage)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        // Insert spreads
        for (spread, percentage) in &stats.spreads {
            sqlx::query(
                r#"
                INSERT INTO usage_spreads (form_id, spread, percentage)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(stats.form_id)
            .bind(spread)
            .bind(percentage)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        // Insert tera types
        for (tera_type, percentage) in &stats.tera_types {
            sqlx::query(
                r#"
                INSERT INTO usage_tera_types (form_id, tera_type, percentage)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(stats.form_id)
            .bind(tera_type)
            .bind(percentage)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_usage_stats(
        &self,
        _format: &str,
        _period: &str,
    ) -> Result<(), RepositoryError> {
        // In the new design with form_id as primary key, we delete all usage stats
        // when syncing new data (since we only store a single format/period at a time)
        sqlx::query("DELETE FROM usage_stats")
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn migrate(&self) -> Result<(), RepositoryError> {
        // Create pokemon_species table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS pokemon_species (
                species_id INTEGER PRIMARY KEY,
                name VARCHAR(100) NOT NULL,
                name_ja VARCHAR(100),
                is_legendary BOOLEAN NOT NULL DEFAULT FALSE,
                is_mythical BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP NOT NULL DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Create pokemon_forms table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS pokemon_forms (
                form_id INTEGER PRIMARY KEY,
                species_id INTEGER NOT NULL REFERENCES pokemon_species(species_id),
                form_name VARCHAR(100),
                fullname VARCHAR(100) NOT NULL,
                fullname_ja VARCHAR(100),
                type1 VARCHAR(20) NOT NULL,
                type2 VARCHAR(20),
                hp INTEGER NOT NULL,
                attack INTEGER NOT NULL,
                defense INTEGER NOT NULL,
                sp_attack INTEGER NOT NULL,
                sp_defense INTEGER NOT NULL,
                speed INTEGER NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP NOT NULL DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Create index
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_pokemon_forms_species ON pokemon_forms(species_id)",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Create usage_stats table (3NF compliant, form_id as primary key)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS usage_stats (
                form_id INTEGER PRIMARY KEY REFERENCES pokemon_forms(form_id),
                format VARCHAR(50) NOT NULL,
                period VARCHAR(10) NOT NULL,
                raw_count INTEGER NOT NULL,
                usage DOUBLE PRECISION NOT NULL,
                updated_at TIMESTAMP DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Create usage_abilities table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS usage_abilities (
                form_id INTEGER NOT NULL REFERENCES usage_stats(form_id) ON DELETE CASCADE,
                ability_name VARCHAR(100) NOT NULL,
                percentage DOUBLE PRECISION NOT NULL,
                PRIMARY KEY (form_id, ability_name)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Create usage_items table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS usage_items (
                form_id INTEGER NOT NULL REFERENCES usage_stats(form_id) ON DELETE CASCADE,
                item_name VARCHAR(100) NOT NULL,
                percentage DOUBLE PRECISION NOT NULL,
                PRIMARY KEY (form_id, item_name)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Create usage_moves table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS usage_moves (
                form_id INTEGER NOT NULL REFERENCES usage_stats(form_id) ON DELETE CASCADE,
                move_name VARCHAR(100) NOT NULL,
                percentage DOUBLE PRECISION NOT NULL,
                PRIMARY KEY (form_id, move_name)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Create usage_spreads table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS usage_spreads (
                form_id INTEGER NOT NULL REFERENCES usage_stats(form_id) ON DELETE CASCADE,
                spread VARCHAR(200) NOT NULL,
                percentage DOUBLE PRECISION NOT NULL,
                PRIMARY KEY (form_id, spread)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Create usage_tera_types table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS usage_tera_types (
                form_id INTEGER NOT NULL REFERENCES usage_stats(form_id) ON DELETE CASCADE,
                tera_type VARCHAR(20) NOT NULL,
                percentage DOUBLE PRECISION NOT NULL,
                PRIMARY KEY (form_id, tera_type)
            )
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Create indexes for format/period lookups
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_usage_stats_format_period ON usage_stats(format, period)")
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // NOTE: users, teams, user_pokemon, team_pokemon tables are managed by the backend application
        // They are NOT created here to avoid schema conflicts

        log::info!("✅ Database migrations completed");

        Ok(())
    }
}
