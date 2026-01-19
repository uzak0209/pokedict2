.PHONY: help install dev build test clean lint format docker-up docker-down kill-ports

# ポートチェック用のヘルパー関数
define kill_port
	@echo "🔍 ポート $(1) をチェック中..."
	@if lsof -ti:$(1) > /dev/null 2>&1; then \
		echo "⚠️  ポート $(1) は既に使用されています。プロセスを停止します..."; \
		lsof -ti:$(1) | xargs kill -9 2>/dev/null || true; \
		sleep 1; \
		echo "✅ ポート $(1) を解放しました"; \
	fi
endef

# デフォルトターゲット
help:
	@echo "ポケモン構築ビルダー - 利用可能なコマンド"
	@echo ""
	@echo "セットアップ:"
	@echo "  make install          - 依存関係をインストール（frontend + backend）"
	@echo "  make install-frontend - フロントエンドの依存関係をインストール"
	@echo "  make install-backend  - バックエンドの依存関係をインストール"
	@echo ""
	@echo "開発:"
	@echo "  make dev              - 開発サーバーを起動（frontend + backend）"
	@echo "  make dev-frontend     - フロントエンド開発サーバーを起動"
	@echo "  make dev-backend      - バックエンド開発サーバーを起動"
	@echo "  make kill-ports       - 使用中のポートを解放（5173, 8080）"
	@echo ""
	@echo "ビルド:"
	@echo "  make build            - プロジェクト全体をビルド"
	@echo "  make build-frontend   - フロントエンドをビルド"
	@echo "  make build-backend    - バックエンドをビルド"
	@echo ""
	@echo "テスト:"
	@echo "  make test             - 全テストを実行"
	@echo "  make test-backend     - バックエンドのテストを実行"
	@echo ""
	@echo "Lint & Format:"
	@echo "  make lint             - Lint チェックを実行"
	@echo "  make lint-backend     - バックエンドのLintチェック"
	@echo "  make format           - コードフォーマットを実行"
	@echo "  make format-backend   - バックエンドのフォーマット"
	@echo ""
	@echo "クリーンアップ:"
	@echo "  make clean            - ビルド成果物を削除"
	@echo "  make clean-frontend   - フロントエンドのビルド成果物を削除"
	@echo "  make clean-backend    - バックエンドのビルド成果物を削除"
	@echo ""
	@echo "Docker（本番モード）:"
	@echo "  make docker-build     - Dockerイメージをビルド"
	@echo "  make docker-up        - コンテナを起動（バックグラウンド）"
	@echo "  make docker-down      - コンテナを停止"
	@echo "  make docker-logs      - ログを表示"
	@echo "  make docker-clean     - コンテナとボリュームを削除"
	@echo ""
	@echo "Docker（開発モード）:"
	@echo "  make docker-dev       - 開発環境を起動（ホットリロード）"
	@echo "  make docker-dev-build - 開発環境のイメージをビルド"
	@echo "  make docker-logs-dev  - 開発環境のログを表示"

# セットアップ
install: install-frontend install-backend
	@echo "✅ すべての依存関係をインストールしました"

install-frontend:
	@echo "📦 フロントエンドの依存関係をインストール中..."
	cd frontend && npm install

install-backend:
	@echo "🦀 バックエンドの依存関係を確認中..."
	cd backend && cargo check

# ポート解放
kill-ports:
	@echo "🔄 開発用ポートを解放中..."
	$(call kill_port,5173)
	$(call kill_port,8080)
	@echo "✅ すべてのポートを解放しました"

# 開発サーバー
dev: kill-ports
	@echo "🚀 開発サーバーを起動します"
	@echo "フロントエンド: http://localhost:5173"
	@echo "バックエンド: http://localhost:8080"
	@make -j2 dev-frontend-run dev-backend-run

dev-frontend:
	$(call kill_port,5173)
	@make dev-frontend-run

dev-frontend-run:
	@echo "⚛️  フロントエンド開発サーバーを起動中..."
	cd frontend && npm run dev

dev-backend:
	$(call kill_port,8080)
	@make dev-backend-run

dev-backend-run:
	@echo "🦀 バックエンド開発サーバーを起動中..."
	cd backend && cargo run

# ビルド
build: build-frontend build-backend
	@echo "✅ プロジェクト全体のビルドが完了しました"

build-frontend:
	@echo "⚛️  フロントエンドをビルド中..."
	cd frontend && npm run build

build-backend:
	@echo "🦀 バックエンドをビルド中..."
	cd backend && cargo build --release

# テスト
test: test-backend
	@echo "✅ すべてのテストが完了しました"

test-backend:
	@echo "🧪 バックエンドのテストを実行中..."
	cd backend && cargo test --verbose

# Lint & Format
lint: lint-backend
	@echo "✅ Lintチェックが完了しました"

lint-backend:
	@echo "🔍 バックエンドのLintチェック中..."
	cd backend && cargo clippy --all-targets --all-features -- -D warnings -A dead_code

format: format-backend
	@echo "✅ フォーマットが完了しました"

format-backend:
	@echo "✨ バックエンドのフォーマット中..."
	cd backend && cargo fmt --all

format-check:
	@echo "🔍 フォーマットチェック中..."
	cd backend && cargo fmt --all --check

# クリーンアップ
clean: clean-frontend clean-backend
	@echo "✅ クリーンアップが完了しました"

clean-frontend:
	@echo "🧹 フロントエンドのビルド成果物を削除中..."
	cd frontend && rm -rf dist node_modules/.vite

clean-backend:
	@echo "🧹 バックエンドのビルド成果物を削除中..."
	cd backend && cargo clean

# Docker操作
docker-up:
	@echo "🐳 Docker コンテナを起動中（本番モード）..."
	docker compose up -d

docker-down:
	@echo "🐳 Docker コンテナを停止中..."
	docker compose down

docker-dev:
	@echo "🐳 Docker コンテナを起動中（開発モード）..."
	docker compose -f docker-compose.dev.yml up

docker-dev-build:
	@echo "🐳 Docker イメージをビルド中（開発モード）..."
	docker compose -f docker-compose.dev.yml build

docker-build:
	@echo "🐳 Docker イメージをビルド中（本番モード）..."
	docker compose build

docker-logs:
	@echo "📋 Docker ログを表示中..."
	docker compose logs -f

docker-logs-dev:
	@echo "📋 Docker ログを表示中（開発モード）..."
	docker compose -f docker-compose.dev.yml logs -f

docker-clean:
	@echo "🧹 Docker コンテナとボリュームを削除中..."
	docker compose down -v
	docker compose -f docker-compose.dev.yml down -v

# CI/CDで使用するコマンド
ci: format-check lint test build
	@echo "✅ CI チェックがすべて完了しました"

# 開発環境のセットアップ（初回のみ）
setup: install
	@echo "🎉 開発環境のセットアップが完了しました！"
	@echo ""
	@echo "次のステップ:"
	@echo "  1. make dev          - 開発サーバーを起動"
	@echo "  2. ブラウザで http://localhost:5173 を開く"
