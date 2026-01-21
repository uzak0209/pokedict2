use serde::Serialize;
use sqlx::{FromRow, PgPool};
use typeshare::typeshare;

/// ポケモンマスタデータDTO
#[typeshare]
#[derive(Debug, Serialize, FromRow, Clone)]
pub struct PokemonMasterDto {
    pub form_id: i32,
    pub species_id: i32,
    pub fullname: String,
    pub fullname_ja: Option<String>,
    pub type1: String,
    pub type2: Option<String>,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub sp_attack: i32,
    pub sp_defense: i32,
    pub speed: i32,
    pub usage: Option<f64>,
    #[typeshare(serialized_as = "Option<number>")]
    pub raw_count: Option<i32>,
}

/// 使用率詳細データDTO
#[typeshare]
#[derive(Debug, Serialize, FromRow, Clone)]
pub struct UsageDetailDto {
    pub name: String,
    pub percentage: f64,
}

/// ポケモン使用率統計DTO
#[typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct PokemonUsageStatsDto {
    pub form_id: i32,
    pub moves: Vec<UsageDetailDto>,
    pub items: Vec<UsageDetailDto>,
    pub abilities: Vec<UsageDetailDto>,
    pub tera_types: Vec<UsageDetailDto>,
    pub natures: Vec<UsageDetailDto>,
}

#[derive(Debug, Serialize)]
pub struct PokemonMasterResponseDto {
    pub pokemon: Vec<PokemonMasterDto>,
    pub total: i32,
}

/// ポケモンマスタデータリポジトリ
#[derive(Clone)]
pub struct PokemonMasterRepository {
    pool: PgPool,
}

impl PokemonMasterRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// 全ポケモンのマスタデータを取得（使用率順）
    pub async fn get_all_pokemon(&self) -> Result<Vec<PokemonMasterDto>, sqlx::Error> {
        sqlx::query_as::<_, PokemonMasterDto>(
            r#"
            SELECT 
                pf.form_id,
                pf.species_id,
                pf.fullname,
                pf.fullname_ja,
                pf.type1,
                pf.type2,
                pf.hp,
                pf.attack,
                pf.defense,
                pf.sp_attack,
                pf.sp_defense,
                pf.speed,
                us.usage,
                us.raw_count
            FROM pokemon_forms pf
            LEFT JOIN usage_stats us ON pf.form_id = us.form_id
            ORDER BY COALESCE(us.usage, 0) DESC, pf.fullname ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
    }

    /// 使用率ランキング上位のポケモンを取得
    pub async fn get_top_pokemon(&self, limit: i32) -> Result<Vec<PokemonMasterDto>, sqlx::Error> {
        sqlx::query_as::<_, PokemonMasterDto>(
            r#"
            SELECT 
                pf.form_id,
                pf.species_id,
                pf.fullname,
                pf.fullname_ja,
                pf.type1,
                pf.type2,
                pf.hp,
                pf.attack,
                pf.defense,
                pf.sp_attack,
                pf.sp_defense,
                pf.speed,
                us.usage,
                us.raw_count
            FROM pokemon_forms pf
            INNER JOIN usage_stats us ON pf.form_id = us.form_id
            WHERE us.usage IS NOT NULL
            ORDER BY us.usage DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    /// 名前（英語名または日本語名）からポケモンを検索
    pub async fn find_by_name(&self, name: &str) -> Result<Option<PokemonMasterDto>, sqlx::Error> {
        // 大文字小文字を無視して検索
        // fullname (英語) または fullname_ja (日本語) に一致するものを探す
        sqlx::query_as::<_, PokemonMasterDto>(
            r#"
            SELECT 
                pf.form_id,
                pf.species_id,
                pf.fullname,
                pf.fullname_ja,
                pf.type1,
                pf.type2,
                pf.hp,
                pf.attack,
                pf.defense,
                pf.sp_attack,
                pf.sp_defense,
                pf.speed,
                us.usage,
                us.raw_count
            FROM pokemon_forms pf
            LEFT JOIN usage_stats us ON pf.form_id = us.form_id
            WHERE LOWER(pf.fullname) = LOWER($1) OR pf.fullname_ja = $1
            LIMIT 1
            "#,
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
    }


    /// ポケモンの詳細使用率データを取得
    pub async fn get_usage_stats(&self, form_id: i32) -> Result<PokemonUsageStatsDto, sqlx::Error> {
        // 技の使用率
        let moves = sqlx::query_as::<_, UsageDetailDto>(
            r#"
            SELECT move_name as name, percentage 
            FROM usage_moves 
            WHERE form_id = $1 
            ORDER BY percentage DESC
            LIMIT 20
            "#,
        )
        .bind(form_id)
        .fetch_all(&self.pool)
        .await?;

        // 持ち物の使用率
        let items = sqlx::query_as::<_, UsageDetailDto>(
            r#"
            SELECT item_name as name, percentage 
            FROM usage_items 
            WHERE form_id = $1 
            ORDER BY percentage DESC
            LIMIT 20
            "#,
        )
        .bind(form_id)
        .fetch_all(&self.pool)
        .await?;

        // 特性の使用率
        let abilities = sqlx::query_as::<_, UsageDetailDto>(
            r#"
            SELECT ability_name as name, percentage 
            FROM usage_abilities 
            WHERE form_id = $1 
            ORDER BY percentage DESC
            LIMIT 10
            "#,
        )
        .bind(form_id)
        .fetch_all(&self.pool)
        .await?;

        // テラスタイプの使用率
        let tera_types = sqlx::query_as::<_, UsageDetailDto>(
            r#"
            SELECT tera_type as name, percentage 
            FROM usage_tera_types 
            WHERE form_id = $1 
            ORDER BY percentage DESC
            LIMIT 18
            "#,
        )
        .bind(form_id)
        .fetch_all(&self.pool)
        .await?;

        // 性格の使用率 (spreadsから集計)
        // spread format: "Nature:HP/Atk/Def/SpA/SpD/Spe"
        let natures = sqlx::query_as::<_, UsageDetailDto>(
            r#"
            SELECT 
                split_part(spread, ':', 1) as name,
                SUM(percentage) as percentage
            FROM usage_spreads
            WHERE form_id = $1
            GROUP BY split_part(spread, ':', 1)
            ORDER BY percentage DESC
            LIMIT 10
            "#,
        )
        .bind(form_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(PokemonUsageStatsDto {
            form_id,
            moves,
            items,
            abilities,
            tera_types,
            natures,
        })
    }
}
