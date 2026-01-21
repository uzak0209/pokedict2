use crate::domain::UsageStats;
use crate::repository::PokemonRepository;
use crate::smogon_client::SmogonClient;
use crate::smogon_models::SmogonChaosResponse;
use anyhow::{Context, Result};
use std::sync::Arc;

pub struct SyncSmogonService<R: PokemonRepository + ?Sized> {
    repository: Arc<R>,
    client: SmogonClient,
}

#[derive(Debug, Default)]
pub struct SmogonSyncStats {
    pub format: String,
    pub total_fetched: i32,
    pub mapped_saved: i32,
    pub skipped_not_found: i32,
    pub errors: i32,
}

impl<R: PokemonRepository + ?Sized> SyncSmogonService<R> {
    pub fn new(repository: Arc<R>, client: SmogonClient) -> Self {
        Self { repository, client }
    }

    pub async fn sync_usage_stats(
        &self,
        format: &str,
        period: &str,
        rating: i32,
    ) -> Result<SmogonSyncStats> {
        let mut stats = SmogonSyncStats {
            format: format.to_string(),
            ..Default::default()
        };

        // 1. Smogonからデータを取得
        let chaos_data = self
            .client
            .fetch_chaos_json(format, period, rating)
            .await
            .context("Failed to fetch Smogon data")?;

        let total_count = chaos_data.data.len() as i32;
        stats.total_fetched = total_count;

        log::info!(
            "📥 Fetched {} entries for {}/{}",
            total_count,
            period,
            format
        );

        // 既存データをクリア
        log::info!(
            "🗑️ Clearing existing usage stats for {}/{}...",
            period,
            format
        );
        self.repository
            .delete_usage_stats(format, period)
            .await
            .context("Failed to clear existing usage stats")?;

        // 2. 各エントリを処理
        for (smogon_name, data) in chaos_data.data {
            // 名前正規化
            let normalized_name = SmogonChaosResponse::normalize_name(&smogon_name);

            // DBからform_idを検索
            let form_id_opt = match self
                .repository
                .find_form_id_by_fullname(&normalized_name)
                .await
            {
                Ok(id) => id,
                Err(e) => {
                    log::error!("❌ Database error processing {}: {}", normalized_name, e);
                    stats.errors += 1;
                    continue;
                }
            };

            let form_id = if let Some(id) = form_id_opt {
                id
            } else {
                log::warn!(
                    "⚠️ Skipped: '{}' (normalized: '{}') not found in DB",
                    smogon_name,
                    normalized_name
                );
                stats.skipped_not_found += 1;
                continue;
            };

            // UsageStats作成
            let usage_stats = UsageStats::new(
                format.to_string(),
                period.to_string(),
                form_id,
                data.raw_count,
                data.usage,
                data.abilities,
                data.items,
                data.moves,
                data.spreads,
                data.tera_types,
            );

            // 保存
            if let Err(e) = self.repository.save_usage_stats(&usage_stats).await {
                log::error!(
                    "❌ Failed to save usage stats for {}: {}",
                    normalized_name,
                    e
                );
                stats.errors += 1;
            } else {
                stats.mapped_saved += 1;
            }
        }

        Ok(stats)
    }
}
