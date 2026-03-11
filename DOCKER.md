# Docker 使用ガイド

このプロジェクトは Docker と Docker Compose を使って簡単に起動できます。

## 📦 構成

### サービス

#### 本番環境 (`docker-compose.yml`)
- **frontend**: React + Nginx (ポート 3000)
- **backend**: Rust + Actix-web (ポート 8080)
- **db**: PostgreSQL 16 (ポート 5432)

#### 開発環境 (`docker-compose.dev.yml`)
- **frontend-dev**: Vite 開発サーバー（ホットリロード）
- **backend-dev**: Cargo watch（ホットリロード）
- **db**: PostgreSQL 16

## 🚀 クイックスタート

### 開発環境

ホットリロード対応の開発環境を起動：

```bash
make docker-dev
```

アクセス:
- フロントエンド: http://localhost:5173
- バックエンド: http://localhost:8080
- PostgreSQL: localhost:5432

### 本番環境

本番用にビルドして起動：

```bash
# イメージをビルド
make docker-build

# バックグラウンドで起動
make docker-up
```

アクセス:
- アプリケーション: http://localhost:3000
  - Nginx が React アプリを配信
  - `/api` は自動的にバックエンドにプロキシ

## 📋 よく使うコマンド

### 起動・停止

```bash
# 開発環境を起動（フォアグラウンド）
make docker-dev

# 本番環境を起動（バックグラウンド）
make docker-up

# コンテナを停止
make docker-down
```

### ビルド

```bash
# 本番イメージをビルド
make docker-build

# 開発イメージをビルド
make docker-dev-build

# キャッシュを使わず再ビルド
docker compose build --no-cache
```

### ログとデバッグ

```bash
# 本番環境のログを表示
make docker-logs

# 開発環境のログを表示
make docker-logs-dev

# 特定のサービスのログを表示
docker compose logs -f frontend
docker compose logs -f backend
```

### データベース操作

```bash
# PostgreSQL に接続
docker exec -it pokedict-db psql -U postgres -d pokedict

# データベースのバックアップ
docker exec pokedict-db pg_dump -U postgres pokedict > backup.sql

# バックアップをリストア
docker exec -i pokedict-db psql -U postgres pokedict < backup.sql
```

### クリーンアップ

```bash
# コンテナを停止してボリュームも削除
make docker-clean

# すべての Docker リソースをクリーンアップ
docker system prune -a --volumes
```

## 🔧 トラブルシューティング

### ポートが既に使用されている

別のサービスがポートを使っている場合：

```bash
# ポート番号を変更（docker-compose.yml を編集）
services:
  frontend:
    ports:
      - "8000:80"  # 3000 → 8000 に変更
```

### ビルドエラー

キャッシュをクリアして再ビルド：

```bash
make docker-clean
docker compose build --no-cache
make docker-up
```

### データベース接続エラー

コンテナが起動しているか確認：

```bash
docker compose ps

# データベースログを確認
docker compose logs db
```

### ホットリロードが動作しない

開発環境のボリュームマウントを確認：

```bash
# コンテナを再起動
make docker-down
make docker-dev
```

## 📁 永続化データ

### ボリューム

- `postgres-data`: 本番環境のデータベースデータ
- `postgres-dev-data`: 開発環境のデータベースデータ
- `cargo-cache`: Rust の依存関係キャッシュ
- `target-cache`: Rust のビルドキャッシュ

### データの確認

```bash
# ボリューム一覧
docker volume ls

# ボリュームの詳細
docker volume inspect pokedict2_postgres-data
```

## 🔐 環境変数

### バックエンド

| 変数名 | デフォルト値 | 説明 |
|--------|--------------|------|
| `DATABASE_URL` | `postgres://postgres:password@db:5432/pokedict` | PostgreSQL 接続文字列 |
| `RUST_LOG` | `info` | ログレベル (開発: `debug`) |

### フロントエンド

| 変数名 | デフォルト値 | 説明 |
|--------|--------------|------|
| `VITE_API_URL` | `http://localhost:8080` | バックエンドAPI URL |

環境変数を変更するには `docker-compose.yml` を編集してください。

## 📊 パフォーマンス最適化

### ビルドキャッシュの活用

Dockerfile では multi-stage build を使用しています：

1. 依存関係のみを先にビルド → キャッシュ
2. ソースコード変更時は高速ビルド

### 本番イメージのサイズ削減

- Alpine Linux ベース
- 不要なファイルは `.dockerignore` で除外
- Multi-stage build で最終イメージは最小限

## 🔄 CI/CD との統合

GitHub Actions などで使用する例：

```yaml
name: Docker Build

on: [push]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build Docker images
        run: make docker-build

      - name: Run tests in container
        run: |
          docker compose up -d
          docker compose exec -T backend cargo test
```

## 📚 参考リンク

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [Docker Best Practices](https://docs.docker.com/develop/dev-best-practices/)
