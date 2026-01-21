use crate::pokeapi_client::PokeApiClient;
use crate::repository::PokemonRepository;
use anyhow::{Context, Result};
use std::sync::Arc;

/// 同期統計情報
#[derive(Debug, Default)]
pub struct SyncStats {
    pub species_synced: i32,
    pub forms_synced: i32,
    pub species_skipped: i32,
    #[allow(dead_code)]
    pub forms_skipped: i32,
    pub errors: i32,
}

/// 同期サービス
pub struct SyncService<R: PokemonRepository> {
    repository: Arc<R>,
    api_client: PokeApiClient,
}

impl<R: PokemonRepository> SyncService<R> {
    #[must_use]
    pub fn new(repository: Arc<R>, api_client: PokeApiClient) -> Self {
        Self {
            repository,
            api_client,
        }
    }

    /// 全ポケモンを同期
    pub async fn sync_all_pokemon(&self, limit: Option<i32>) -> Result<SyncStats> {
        let mut stats = SyncStats::default();
        let limit = limit.unwrap_or(10000); // デフォルトは全て

        log::info!("🚀 Starting Pokemon sync (limit: {limit})");

        // Species リストを取得
        let species_list = self
            .api_client
            .fetch_species_list(limit, 0)
            .await
            .context("Failed to fetch species list")?;

        log::info!("📋 Found {} species to sync", species_list.results.len());

        // 各Speciesを同期
        for (index, item) in species_list.results.iter().enumerate() {
            let species_id = PokeApiClient::extract_pokemon_id(&item.url);

            if let Some(id) = species_id {
                log::info!(
                    "⏳ [{}/{}] Syncing species: {} (id: {})",
                    index + 1,
                    species_list.results.len(),
                    item.name,
                    id
                );

                match self.sync_species(id).await {
                    Ok((species_saved, forms_saved)) => {
                        if species_saved {
                            stats.species_synced += 1;
                        } else {
                            stats.species_skipped += 1;
                        }
                        stats.forms_synced += forms_saved;
                        log::info!(
                            "  ✅ Synced {} (species: {}, forms: {})",
                            item.name,
                            if species_saved { "saved" } else { "skipped" },
                            forms_saved
                        );
                    }
                    Err(e) => {
                        stats.errors += 1;
                        log::error!("  ❌ Failed to sync {}: {}", item.name, e);
                    }
                }
            } else {
                log::warn!("  ⚠️  Could not extract species ID from URL: {}", item.url);
                stats.errors += 1;
            }
        }

        log::info!("🎉 Sync completed!");
        log::info!("  Species synced: {}", stats.species_synced);
        log::info!("  Species skipped: {}", stats.species_skipped);
        log::info!("  Forms synced: {}", stats.forms_synced);
        log::info!("  Errors: {}", stats.errors);

        Ok(stats)
    }

    /// 特定のSpeciesとそのフォームを同期
    /// 戻り値: (species was saved, number of forms saved)
    pub async fn sync_species(&self, species_id: i32) -> Result<(bool, i32)> {
        // Species情報を取得
        let species_response = self
            .api_client
            .fetch_species(species_id)
            .await
            .context(format!("Failed to fetch species {species_id}"))?;

        // 日本語名を保存
        let name_ja = species_response
            .names
            .iter()
            .find(|n| n.language.name == "ja" || n.language.name == "ja-Hrkt")
            .map(|n| n.name.clone());

        let pokemon_urls = species_response.get_pokemon_urls();

        // Speciesを保存（既に存在する場合はスキップ）
        let species = species_response.into_domain();
        let species_saved = !self
            .repository
            .species_exists(species_id)
            .await
            .context("Failed to check species existence")?;

        self.repository
            .save_species(&species)
            .await
            .context(format!("Failed to save species {species_id}"))?;

        let mut forms_saved = 0;

        // 各Pokemon（バリエーション）を処理
        for url in pokemon_urls {
            if let Some(pokemon_id) = PokeApiClient::extract_pokemon_id(&url) {
                // Pokemon情報を取得
                let pokemon_response = self
                    .api_client
                    .fetch_pokemon(pokemon_id)
                    .await
                    .context(format!("Failed to fetch pokemon {pokemon_id}"))?;

                // 親pokemonのタイプとstatsを抽出
                let base_stats = Self::extract_base_stats(&pokemon_response);

                // このpokemonのforms配列を取得
                let form_urls = pokemon_response.get_form_urls();

                // フォームが1つだけ（is_default）の場合は、親pokemonのデータを使用
                if form_urls.len() == 1 {
                    // 日本語名は species の名前を使用
                    let fullname_ja = name_ja.clone();

                    // Formを保存
                    let form = pokemon_response.into_domain(species_id, fullname_ja);

                    self.repository
                        .save_form(&form)
                        .await
                        .context(format!("Failed to save form {pokemon_id}"))?;

                    forms_saved += 1;
                } else {
                    // 複数のフォームがある場合は各フォームを取得して保存
                    for form_url in form_urls {
                        if let Some(form_id) = PokeApiClient::extract_pokemon_id(&form_url) {
                            match self.api_client.fetch_pokemon_form(form_id).await {
                                Ok(form_response) => {
                                    let form = form_response.into_domain(
                                        species_id,
                                        base_stats.clone(),
                                        name_ja.clone(),
                                    );

                                    self.repository
                                        .save_form(&form)
                                        .await
                                        .context(format!("Failed to save form {form_id}"))?;

                                    forms_saved += 1;
                                }
                                Err(e) => {
                                    log::warn!("  ⚠️  Failed to fetch form {form_id}: {e}");
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok((species_saved, forms_saved))
    }

    /// PokemonResponseからBaseStatsを抽出
    fn extract_base_stats(pokemon: &crate::models::PokemonResponse) -> crate::domain::BaseStats {
        let mut hp = 0;
        let mut attack = 0;
        let mut defense = 0;
        let mut sp_attack = 0;
        let mut sp_defense = 0;
        let mut speed = 0;

        for stat in &pokemon.stats {
            match stat.stat.name.as_str() {
                "hp" => hp = stat.base_stat,
                "attack" => attack = stat.base_stat,
                "defense" => defense = stat.base_stat,
                "special-attack" => sp_attack = stat.base_stat,
                "special-defense" => sp_defense = stat.base_stat,
                "speed" => speed = stat.base_stat,
                _ => {}
            }
        }

        crate::domain::BaseStats::new(hp, attack, defense, sp_attack, sp_defense, speed)
    }
}
