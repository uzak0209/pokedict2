DATABASE_URL ?= postgres://postgres:password@localhost:5432/pokedict
SMOGON_FORMAT ?= gen9bssregj
SMOGON_PERIOD ?= 2025-12
SMOGON_RATING ?= 0

.PHONY: all build sync-pokeapi sync-smogon check-names truncate force

all: sync-pokeapi sync-smogon

build:
	cargo build --release --bin sync-pokeapi

# 全データ同期（PokeAPI全取得 + 欠損修復）
sync-pokeapi: build
	@echo "🔄 Starting Full PokeAPI Sync..."
	./target/release/sync-pokeapi --database-url "$(DATABASE_URL)"
	@echo "🔧 Running Repair Script..."
	export DATABASE_URL="$(DATABASE_URL)" && ./repair_forms.sh
	@echo "✨ PokeAPI Sync Completed!"

# Smogonデータ同期
sync-smogon: build
	@echo "⚔️ Starting Smogon Sync ($(SMOGON_FORMAT) / $(SMOGON_PERIOD))..."
	./target/release/sync-pokeapi --database-url "$(DATABASE_URL)" \
		--smogon-format $(SMOGON_FORMAT) \
		--smogon-period $(SMOGON_PERIOD) \
		--smogon-rating $(SMOGON_RATING) \
		--smogon-only
	@echo "✨ Smogon Sync Completed!"

# マッピング確認用
check-names:
	cargo run --bin check_names -- "$(DATABASE_URL)" "ninetales"

# DBクリア（注意！）
truncate:
	cargo run --bin truncate_db -- "$(DATABASE_URL)"
