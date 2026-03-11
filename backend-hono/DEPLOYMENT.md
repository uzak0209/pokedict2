# Deployment Guide

このプロジェクトはCloudflare Workersへ自動デプロイされます。

## 🚀 自動デプロイ

### 環境とブランチ戦略

| 環境 | ブランチ | デプロイ | 承認 | URL |
|------|---------|---------|------|-----|
| **Development** | `dev/*` | なし | - | ローカルのみ |
| **Staging** | `staging`, `dev` | 自動 | 不要 | https://pokedict-backend-staging.workers.dev |
| **Production** | `main` | 自動 | **必要** | https://pokedict-backend-prod.workers.dev |

### デプロイフロー

**Staging環境:**
```
staging または dev ブランチへPush
  ↓
自動デプロイ（承認不要）
```

**Production環境:**
```
main ブランチへPush
  ↓
レビュワーによる承認待ち
  ↓
承認後に自動デプロイ
```

**Pull Request:**
```
Lint & Type Check のみ実行
```

## ⚙️ 初期セットアップ

### 1. Cloudflare アカウント設定

1. [Cloudflare Dashboard](https://dash.cloudflare.com/)にログイン
2. Workers & Pages > Overview に移動
3. Account ID をコピー

### 2. API Token の作成

1. Cloudflare Dashboard > My Profile > API Tokens
2. "Create Token" をクリック
3. "Edit Cloudflare Workers" テンプレートを選択
4. 権限を確認して "Continue to summary"
5. "Create Token" をクリックしてトークンをコピー

### 3. GitHub Environments の設定

**重要**: Production環境への承認フローを設定します。

詳細な手順は [.github/ENVIRONMENT_SETUP.md](../.github/ENVIRONMENT_SETUP.md) を参照してください。

**概要:**
1. `Settings` > `Environments` で `staging` と `production` を作成
2. Production環境に承認者を設定（2名推奨）
3. デプロイブランチを制限

### 4. GitHub Secrets の設定

リポジトリの Settings > Secrets and variables > Actions で以下を追加：

```
CLOUDFLARE_API_TOKEN=<your-api-token>
CLOUDFLARE_ACCOUNT_ID=<your-account-id>
```

### 5. Cloudflare Secrets の設定

**重要**: Staging環境とProduction環境で異なる値を使用してください！

#### Staging環境

```bash
npx wrangler secret put DATABASE_URL --env staging
# 入力例: postgresql://user:pass@staging-db.region.neon.tech:5432/pokedict_staging

npx wrangler secret put JWT_SECRET --env staging
# 注意: Production と異なる値を使用
npx wrangler secret put ALLOWED_ORIGIN --env production
npx wrangler secret put GEMINI_API_KEY --env production

# Staging環境
npx wrangler secret put DATABASE_URL --env staging
npx wrangler secret put JWT_SECRET --env staging
npx wrangler secret put ALLOWED_ORIGIN --env staging
npx wrangler secret put GEMINI_API_KEY --env staging
```

**重要な値:**

- `DATABASE_URL`: PostgreSQL接続文字列（Neon、Supabase等の外部DBを推奨）
  - 例: `postgresql://user:password@host.region.provider.com:5432/dbname?sslmode=require`
- `JWT_SECRET`: 32文字以上のランダムな文字列
- `ALLOWED_ORIGIN`: フロントエンドのURL（例: `https://yourapp.com`）
- `GEMINI_API_KEY`: Google Gemini API キー（オプション）

## 🛠️ ローカル開発

### Node.js サーバーで開発

```bash
npm run dev:node
```

### Cloudflare Workers エミュレーターで開発

```bash
npm run dev
```

Wranglerの開発サーバーが起動し、本番環境に近い環境でテストできます。

## 📦 手動デプロイ

必要に応じて手動でデプロイ可能：

```bash
# Staging環境
npm run deploy:staging

# Production環境
npm run deploy:production
```

## 🗄️ データベース設定

### 推奨: 外部PostgreSQLサービス

Cloudflare Workersから接続可能な外部PostgreSQLを使用：

1. **Neon** (https://neon.tech)
   - Serverless Postgres
   - 無料プランあり
   - 低レイテンシー

2. **Supabase** (https://supabase.com)
   - PostgreSQL + 追加機能
   - 無料プランあり

3. **Render** (https://render.com)
   - PostgreSQL ホスティング

### データベース移行

マイグレーションはローカルまたはCI/CDで実行：

```bash
npm run db:generate  # Drizzleスキーマからマイグレーション生成
npm run db:migrate   # マイグレーション実行
```

## 🔍 モニタリング

### Cloudflare Dashboard

1. Workers & Pages > Overview
2. デプロイされたWorkerを選択
3. Metrics タブでパフォーマンスを確認

### ログ確認

```bash
# リアルタイムログ
npx wrangler tail --env production

# Staging環境
npx wrangler tail --env staging
```

## 🚨 トラブルシューティング

### デプロイが失敗する

1. GitHub Secrets が正しく設定されているか確認
2. wrangler.toml の設定を確認
3. GitHub Actions のログを確認

### データベース接続エラー

1. DATABASE_URL が正しく設定されているか確認
2. データベースがCloudflare Workersからアクセス可能か確認
3. SSL接続が有効か確認（`?sslmode=require`）

### 環境変数の確認

```bash
# 設定されているsecretsを確認
npx wrangler secret list --env production
```

## 📝 CI/CD ワークフロー

`.github/workflows/deploy.yml` で定義：

1. **Lint & Type Check**
   - ESLint実行
   - TypeScript型チェック
   - Prettier確認

2. **Deploy Staging** (devブランチ)
   - 依存関係インストール
   - Staging環境へデプロイ

3. **Deploy Production** (mainブランチ)
   - 依存関係インストール
   - Production環境へデプロイ

## 🔐 セキュリティ

- 全てのsecretsはCloudflare Workersの暗号化されたストレージに保存
- GitHub Secretsは暗号化されて保存
- HTTPS通信のみ許可
- CORS設定で許可されたオリジンのみアクセス可能

## 📚 関連ドキュメント

- [Cloudflare Workers Docs](https://developers.cloudflare.com/workers/)
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/)
- [Hono Cloudflare Workers Guide](https://hono.dev/getting-started/cloudflare-workers)
