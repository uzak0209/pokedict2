mod domain;
mod form_localization;
mod models;
mod pokeapi_client;
mod repository;
mod smogon_client;
mod smogon_models;
mod usecase;

use anyhow::{Context, Result};
use clap::Parser;
use pokeapi_client::PokeApiClient;
use repository::{PokemonRepository, PostgresRepository};
use smogon_client::SmogonClient;
use std::sync::Arc;
use usecase::{SyncService, SyncSmogonService};

/// PokeAPIからポケモンデータを同期するCLIツール
#[derive(Parser, Debug)]
#[command(name = "sync-pokeapi")]
#[command(about = "Sync Pokemon data from PokeAPI to PostgreSQL", long_about = None)]
struct Args {
    /// データベース接続URL
    #[arg(long)]
    database_url: String,

    /// 同期するポケモンの最大数（デフォルト: 全て）
    #[arg(short, long)]
    limit: Option<i32>,

    /// 特定のspecies IDのみ同期
    #[arg(short, long)]
    species_id: Option<i32>,

    /// Smogonフォーマット（例: gen9bssregj）
    #[arg(long)]
    smogon_format: Option<String>,

    /// Smogon期間（例: 2025-12）
    #[arg(long)]
    smogon_period: Option<String>,

    /// Smogonレーティング閾値（デフォルト: 1500）
    #[arg(long, default_value = "1500")]
    smogon_rating: i32,

    /// Smogon同期のみを実行（PokeAPI同期をスキップ）
    #[arg(long)]
    smogon_only: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // ロガーを初期化
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // コマンドライン引数をパース
    let args = Args::parse();

    log::info!("🔌 Connecting to database...");

    // データベース接続
    let pool = sqlx::PgPool::connect(&args.database_url)
        .await
        .context("Failed to connect to database")?;

    log::info!("✅ Connected to database");

    // リポジトリとクライアントを作成
    let repository = Arc::new(PostgresRepository::new(pool));
    let api_client = PokeApiClient::new();
    let smogon_client = SmogonClient::new();

    // マイグレーションを実行
    log::info!("🔧 Running migrations...");
    repository
        .migrate()
        .await
        .context("Failed to run migrations")?;

    // Smogon同期を実行
    if let (Some(format), Some(period)) = (&args.smogon_format, &args.smogon_period) {
        log::info!(
            "⚔️ Syncing Smogon stats for {}/{} (rating: {})",
            period,
            format,
            args.smogon_rating
        );
        let smogon_service = SyncSmogonService::new(repository.clone(), smogon_client);

        match smogon_service
            .sync_usage_stats(format, period, args.smogon_rating)
            .await
        {
            Ok(stats) => {
                log::info!("📊 Smogon Sync Statistics:");
                log::info!("  Format: {}", stats.format);
                log::info!("  Total fetched: {}", stats.total_fetched);
                log::info!("  Mapped & Saved: {}", stats.mapped_saved);
                log::info!("  Skipped (not found): {}", stats.skipped_not_found);
                log::info!("  Errors: {}", stats.errors);
            }
            Err(e) => {
                log::error!("❌ Smogon sync failed: {e}");
                return Err(e);
            }
        }

        // Smogon同期のみの場合はここで終了
        if args.smogon_only {
            log::info!("✨ All done!");
            return Ok(());
        }
    }

    // 同期サービスを作成
    let sync_service = SyncService::new(repository, api_client);

    // 同期を実行
    if let Some(species_id) = args.species_id {
        // 特定のspeciesのみ同期
        log::info!("🎯 Syncing specific species: {species_id}");

        match sync_service.sync_species(species_id).await {
            Ok((species_saved, forms_saved)) => {
                log::info!("🎉 Sync completed!");
                log::info!(
                    "  Species: {}",
                    if species_saved { "saved" } else { "skipped" }
                );
                log::info!("  Forms synced: {forms_saved}");
            }
            Err(e) => {
                log::error!("❌ Sync failed: {e}");
                return Err(e);
            }
        }
    } else {
        // 全てのポケモンを同期
        match sync_service.sync_all_pokemon(args.limit).await {
            Ok(stats) => {
                log::info!("📊 Final Statistics:");
                log::info!("  Species synced: {}", stats.species_synced);
                log::info!("  Species skipped: {}", stats.species_skipped);
                log::info!("  Forms synced: {}", stats.forms_synced);
                log::info!("  Errors: {}", stats.errors);
            }
            Err(e) => {
                log::error!("❌ Sync failed: {e}");
                return Err(e);
            }
        }
    }

    log::info!("✨ All done!");

    Ok(())
}
