# Database Migration Guide

このプロジェクトでは、Drizzle ORMを使用してD1データベースのスキーマを管理します。

## マイグレーションの作成

スキーマ（`src/db/schema.ts`）を変更した後、以下のコマンドでマイグレーションファイルを生成します：

```bash
npm run db:generate
```

これにより、`drizzle/` ディレクトリにSQLマイグレーションファイルが生成されます（例：`0001_xxxxx.sql`）。

## マイグレーションの適用

### ローカル環境

```bash
npm run db:migrate:local
```

ローカルD1データベース（`.wrangler/state/v3/d1/`）にマイグレーションを適用します。

### Staging環境

```bash
npm run db:migrate:staging
```

CloudflareのStaging D1データベースにマイグレーションを適用します。

### Production環境

```bash
npm run db:migrate:production
```

CloudflareのProduction D1データベースにマイグレーションを適用します。

**⚠️ 注意**: Productionへの適用は慎重に行ってください。

## 新しいマイグレーションの追加

1. `src/db/schema.ts` を編集
2. `npm run db:generate` で新しいマイグレーションを生成
3. 生成されたSQLファイルを確認
4. ローカルでテスト: `npm run db:migrate:local`
5. Stagingで確認: `npm run db:migrate:staging`
6. 問題なければProduction適用: `npm run db:migrate:production`

## package.json スクリプト更新が必要な場合

新しいマイグレーションファイルが生成された場合（例：`0001_new_migration.sql`）、`package.json`のマイグレーションスクリプトを更新してください：

```json
"db:migrate:local": "wrangler d1 execute DB --local --file=./drizzle/0001_new_migration.sql",
"db:migrate:staging": "wrangler d1 execute pokedict-staging --remote --file=./drizzle/0001_new_migration.sql --env staging",
"db:migrate:production": "wrangler d1 execute pokedict-production --remote --file=./drizzle/0001_new_migration.sql --env production"
```

## Drizzle Studio

データベースの内容をGUIで確認するには：

```bash
npm run db:studio
```

ブラウザでDrizzle Studioが開き、データベースの内容を確認・編集できます。

## ロールバック

D1はマイグレーションのロールバック機能を提供していません。ロールバックが必要な場合：

1. 元のスキーマに戻すマイグレーションを手動で作成
2. または、データベースをバックアップから復元

## トラブルシューティング

### テーブルが既に存在する

```
table `xxx` already exists
```

既にテーブルが存在する場合、以下の方法で解決できます：

1. **データを保持**: マイグレーションをスキップし、そのまま使用
2. **リセット**: テーブルを削除してから再適用
   ```bash
   wrangler d1 execute DB --local --command="DROP TABLE xxx;"
   npm run db:migrate:local
   ```

### マイグレーションファイルが見つからない

`package.json`のマイグレーションスクリプトで指定しているファイル名を確認してください。`drizzle/`ディレクトリ内の最新のマイグレーションファイル名と一致している必要があります。
