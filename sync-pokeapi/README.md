# Sync-PokeAPI

PokeAPIからポケモンデータを取得し、Smogon Usage Statsを同期して、PostgreSQLデータベースに保存するCLIツール。

## 機能

- PokeAPIから全ポケモンの種族情報とフォーム情報を取得
- Smogon Usage Stats（技、特性、持ち物、EVスプレッド、テラスタイプ）を取得
- PostgreSQLデータベースに保存
- DDD/クリーンアーキテクチャに基づいた設計
- レート制限対応

## 使い方

### PokeAPIからポケモンデータを同期

```bash
# ビルド
cargo build --release

# ヘルプ表示
./target/release/sync-pokeapi --help

# 全てのポケモンを同期
./target/release/sync-pokeapi --database-url postgres://postgres:password@localhost:5432/pokedict

# 最初の10種類のみ同期
./target/release/sync-pokeapi --database-url postgres://postgres:password@localhost:5432/pokedict --limit 10

# 特定の種族のみ同期
./target/release/sync-pokeapi --database-url postgres://postgres:password@localhost:5432/pokedict --species-id 25
```

### Smogon Usage Statsを同期

```bash
# gen9bssregh（Regulation H）の使用統計を同期（レーティング1500以上）
./target/release/sync-pokeapi \
  --database-url postgres://postgres:password@localhost:5432/pokedict \
  --smogon-format gen9bssregh \
  --smogon-period 2024-12 \
  --smogon-rating 1500

# レーティング0（全プレイヤー）のデータを取得
./target/release/sync-pokeapi \
  --database-url postgres://postgres:password@localhost:5432/pokedict \
  --smogon-format gen9bssregh \
  --smogon-period 2024-12 \
  --smogon-rating 0

# PokeAPIとSmogonを同時に同期
./target/release/sync-pokeapi \
  --database-url postgres://postgres:password@localhost:5432/pokedict \
  --limit 100 \
  --smogon-format gen9bssregh \
  --smogon-period 2024-12
```

## データベーススキーマ（第3正規形）

自動的にテーブルが作成されます：

### 基本テーブル
- `pokemon_species`: ポケモン種族情報
- `pokemon_forms`: ポケモンフォーム情報

### Usage Statsテーブル（第3正規形）
- `usage_stats`: Smogon使用統計のメインテーブル
  - PRIMARY KEY: form_id（単一のformat/periodのデータのみを保存）
  - format, period は通常のカラムとして保持
  - raw_count, usageなどの集計情報
- `usage_abilities`: 特性データ（PRIMARY KEY: form_id, ability_name）
- `usage_items`: 持ち物データ（PRIMARY KEY: form_id, item_name）
- `usage_moves`: 技データ（PRIMARY KEY: form_id, move_name）
- `usage_spreads`: EVスプレッドデータ（PRIMARY KEY: form_id, spread）
- `usage_tera_types`: テラスタイプデータ（PRIMARY KEY: form_id, tera_type）

各詳細テーブルは usage_stats(form_id) への外部キー制約（ON DELETE CASCADE）を持ちます。

**注意**: この設計では、データベースには常に単一のformat/periodのデータのみが保存されます。新しいformat/periodのデータを同期すると、既存のデータは上書きされます。

### 旧スキーマからの移行

既存のデータベースで旧スキーマ（JSONBを使用）を使用している場合は、以下のコマンドで移行できます：

```bash
# データベースに接続して移行スクリプトを実行
psql postgres://postgres:password@localhost:5432/pokedict -f migrate_to_3nf.sql
```

移行スクリプトは以下の処理を行います：
1. 既存データのバックアップを作成（usage_stats_backup）
2. 旧スキーマからのデータを新しい正規化されたテーブルに移行
3. 移行結果のサマリーを表示

## 利用可能なSmogonフォーマット

- `gen9bssregh`: Generation 9 Battle Stadium Singles Regulation H
- `gen9bssregg`: Generation 9 Battle Stadium Singles Regulation G
- その他のフォーマットは https://www.smogon.com/stats/ を参照
