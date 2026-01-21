use crate::smogon_models::SmogonChaosResponse;
use anyhow::{Context, Result};
use std::time::Duration;

const SMOGON_BASE_URL: &str = "https://www.smogon.com/stats";
const REQUEST_DELAY_MS: u64 = 100;

/// Smogon Stats HTTPクライアント
pub struct SmogonClient {
    client: reqwest::Client,
}

impl SmogonClient {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Chaos JSONを取得
    /// format: "gen9bssregj", "gen9ou", etc.
    /// period: "2025-12", "2025-01", etc.
    /// rating: 0, 1500, 1630, 1760, etc.
    pub async fn fetch_chaos_json(
        &self,
        format: &str,
        period: &str,
        rating: i32,
    ) -> Result<SmogonChaosResponse> {
        let url = format!(
            "{}/{}/chaos/{}-{}.json",
            SMOGON_BASE_URL, period, format, rating
        );

        log::info!("📡 Fetching Smogon stats: {url}");

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context(format!("Failed to fetch Smogon stats from {url}"))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Smogon API returned status {}: {}",
                response.status(),
                url
            ));
        }

        let chaos_data = response
            .json::<SmogonChaosResponse>()
            .await
            .context("Failed to parse Smogon chaos JSON")?;

        log::info!(
            "✅ Fetched {} Pokemon from {} ({} battles)",
            chaos_data.data.len(),
            chaos_data.info.metagame,
            chaos_data.info.number_of_battles
        );

        // レート制限対策
        tokio::time::sleep(Duration::from_millis(REQUEST_DELAY_MS)).await;

        Ok(chaos_data)
    }
}

impl Default for SmogonClient {
    fn default() -> Self {
        Self::new()
    }
}
