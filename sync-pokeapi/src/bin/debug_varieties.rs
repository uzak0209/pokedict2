use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SpeciesResponse {
    pub id: i32,
    pub name: String,
    pub varieties: Vec<VarietyEntry>,
}

#[derive(Debug, Deserialize)]
pub struct VarietyEntry {
    pub is_default: bool,
    pub pokemon: NamedAPIResource,
}

#[derive(Debug, Deserialize)]
pub struct NamedAPIResource {
    pub name: String,
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://pokeapi.co/api/v2/pokemon-species/38/";

    println!("Fetching {}", url);
    let resp = client
        .get(url)
        .send()
        .await?
        .json::<SpeciesResponse>()
        .await?;

    println!("Species: {} (id: {})", resp.name, resp.id);
    println!("Varieties count: {}", resp.varieties.len());
    for v in resp.varieties {
        println!(
            " - {} (default: {}) -> {}",
            v.pokemon.name, v.is_default, v.pokemon.url
        );
    }

    Ok(())
}
