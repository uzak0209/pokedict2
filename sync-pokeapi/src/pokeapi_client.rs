use crate::models::{PokemonFormResponse, PokemonListResponse, PokemonResponse, SpeciesResponse};
use anyhow::{Context, Result};
use std::time::Duration;

const POKEAPI_BASE_URL: &str = "https://pokeapi.co/api/v2";
const REQUEST_DELAY_MS: u64 = 100; // レート制限対策

/// PokeAPI HTTPクライアント
#[derive(Clone)]
pub struct PokeApiClient {
    client: reqwest::Client,
}

impl PokeApiClient {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Pokemon Species リストを取得
    pub async fn fetch_species_list(&self, limit: i32, offset: i32) -> Result<PokemonListResponse> {
        let url = format!("{POKEAPI_BASE_URL}/pokemon-species?limit={limit}&offset={offset}");

        log::debug!("Fetching species list: {url}");

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch species list")?
            .json::<PokemonListResponse>()
            .await
            .context("Failed to parse species list response")?;

        // レート制限対策
        tokio::time::sleep(Duration::from_millis(REQUEST_DELAY_MS)).await;

        Ok(response)
    }

    /// Pokemon Species の詳細を取得
    pub async fn fetch_species(&self, species_id: i32) -> Result<SpeciesResponse> {
        let url = format!("{POKEAPI_BASE_URL}/pokemon-species/{species_id}");

        log::debug!("Fetching species {species_id}: {url}");

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context(format!("Failed to fetch species {species_id}"))?
            .json::<SpeciesResponse>()
            .await
            .context(format!("Failed to parse species {species_id} response"))?;

        // レート制限対策
        tokio::time::sleep(Duration::from_millis(REQUEST_DELAY_MS)).await;

        Ok(response)
    }

    /// Pokemon の詳細を取得
    pub async fn fetch_pokemon(&self, pokemon_id: i32) -> Result<PokemonResponse> {
        let url = format!("{POKEAPI_BASE_URL}/pokemon/{pokemon_id}");

        log::debug!("Fetching pokemon {pokemon_id}: {url}");

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context(format!("Failed to fetch pokemon {pokemon_id}"))?
            .json::<PokemonResponse>()
            .await
            .context(format!("Failed to parse pokemon {pokemon_id} response"))?;

        // レート制限対策
        tokio::time::sleep(Duration::from_millis(REQUEST_DELAY_MS)).await;

        Ok(response)
    }

    /// Pokemon Form の詳細を取得
    pub async fn fetch_pokemon_form(&self, form_id: i32) -> Result<PokemonFormResponse> {
        let url = format!("{POKEAPI_BASE_URL}/pokemon-form/{form_id}");

        log::debug!("Fetching pokemon-form {form_id}: {url}");

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context(format!("Failed to fetch pokemon-form {form_id}"))?
            .json::<PokemonFormResponse>()
            .await
            .context(format!("Failed to parse pokemon-form {form_id} response"))?;

        // レート制限対策
        tokio::time::sleep(Duration::from_millis(REQUEST_DELAY_MS)).await;

        Ok(response)
    }

    /// URL から Pokemon ID を抽出
    #[must_use]
    pub fn extract_pokemon_id(url: &str) -> Option<i32> {
        url.trim_end_matches('/')
            .split('/')
            .next_back()
            .and_then(|s| s.parse::<i32>().ok())
    }
}

impl Default for PokeApiClient {
    fn default() -> Self {
        Self::new()
    }
}
