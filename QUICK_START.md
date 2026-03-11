# 🚀 クイックスタートガイド

最速でCloudflareへデプロイする手順です。

## 📋 前提条件チェック

```bash
# Wranglerがインストールされているか確認
wrangler --version  # ✅ 4.72.0

# Cloudflareにログインしているか確認
wrangler whoami  # ✅ ログイン済み
```

---

## ステップ1: Account ID の確認

```bash
wrangler whoami
```

出力からAccount IDをメモ：

```
┌──────────────────────────────────┬──────────────────────────────────┐
│ Account Name                     │ Account ID                       │
├──────────────────────────────────┼──────────────────────────────────┤
│ Your Account                     │ e3800962ed5e416e565f49c823868cf3 │
└──────────────────────────────────┴──────────────────────────────────┘
```

📝 **Account ID**: `e3800962ed5e416e565f49c823868cf3`

---

## ステップ2: データベースのセットアップ

### オプション A: Neon (推奨)

1. https://neon.tech/ でアカウント作成
2. **New Project** を2つ作成:
   - `pokedict-staging`
   - `pokedict-production`
3. それぞれの接続文字列をコピー

### オプション B: Supabase

1. https://supabase.com/ でアカウント作成
2. プロジェクトを2つ作成
3. Settings > Database から接続文字列をコピー

---

## ステップ3: Cloudflare Secrets の設定

### Staging環境

```bash
cd backend-hono

# DATABASE_URL
wrangler secret put DATABASE_URL --env staging
# 入力: postgresql://user:pass@staging-db.neon.tech:5432/neondb?sslmode=require

# JWT_SECRET (32文字以上)
wrangler secret put JWT_SECRET --env staging
# 入力: staging_jwt_secret_minimum_32_characters_or_more

# ALLOWED_ORIGIN
wrangler secret put ALLOWED_ORIGIN --env staging
# 入力: http://localhost:5173

# GEMINI_API_KEY (オプション)
wrangler secret put GEMINI_API_KEY --env staging
# 入力: (Google AI StudioのAPIキー)
```

### Production環境

```bash
# DATABASE_URL
wrangler secret put DATABASE_URL --env production
# 入力: postgresql://user:pass@prod-db.neon.tech:5432/neondb?sslmode=require

# JWT_SECRET (Stagingと異なる値)
wrangler secret put JWT_SECRET --env production
# 入力: production_jwt_super_secret_32chars_minimum

# ALLOWED_ORIGIN
wrangler secret put ALLOWED_ORIGIN --env production
# 入力: https://yourapp.com

# GEMINI_API_KEY
wrangler secret put GEMINI_API_KEY --env production
```

### 確認

```bash
wrangler secret list --env staging
wrangler secret list --env production
```

---

## ステップ4: データベースマイグレーション

### Stagingデータベース

```bash
cd backend-hono

# .envファイルを編集
echo "DATABASE_URL=postgresql://..." > .env

# マイグレーション実行
npm run db:migrate
```

### Productionデータベース

```bash
# Production DBのURLに変更
echo "DATABASE_URL=postgresql://..." > .env

# マイグレーション実行
npm run db:migrate
```

---

## ステップ5: バックエンドのデプロイ

### Staging

```bash
cd backend-hono
npm run deploy:staging
```

成功すると以下が表示されます：
```
✨ Deployment complete!
https://pokedict-backend-staging.workers.dev
```

### 動作確認

```bash
curl https://pokedict-backend-staging.workers.dev/health
# 出力: {"status":"ok","environment":"staging"}
```

### Production

```bash
npm run deploy:production
```

---

## ステップ6: フロントエンドのデプロイ

### Staging

```bash
cd frontend

# ビルド
VITE_API_URL=https://pokedict-backend-staging.workers.dev npm run build

# デプロイ
wrangler pages deploy dist --project-name=pokedict-frontend-staging
```

### Production

```bash
# ビルド
VITE_API_URL=https://pokedict-backend-prod.workers.dev npm run build

# デプロイ
wrangler pages deploy dist --project-name=pokedict-frontend
```

---

## ステップ7: GitHub Secrets の設定

1. GitHubリポジトリ > **Settings** > **Secrets and variables** > **Actions**
2. **New repository secret** で以下を追加:

### CLOUDFLARE_API_TOKEN

1. https://dash.cloudflare.com/ にログイン
2. **My Profile** > **API Tokens**
3. **Create Token** > **Edit Cloudflare Workers** テンプレート
4. **Create Token** してコピー
5. GitHubに貼り付け

### CLOUDFLARE_ACCOUNT_ID

ステップ1で確認したAccount IDを貼り付け

---

## ステップ8: GitHub Environments の設定

### Staging Environments

1. **Settings** > **Environments**
2. 以下を作成:
   - `staging` (backend用)
   - `staging-frontend` (frontend用)
3. 設定: Deployment branches = `staging`, `dev`

### Production Environments

1. 以下を作成:
   - `production` (backend用)
   - `production-frontend` (frontend用)
2. 設定:
   - ✅ **Required reviewers**: 2名
   - Deployment branches = `main` のみ

---

## ステップ9: 自動デプロイのテスト

### Staging

```bash
# devブランチで変更
git checkout dev
git commit --allow-empty -m "test: auto deploy"
git push origin dev
```

GitHub Actionsで自動デプロイが実行されます。

### Production

```bash
# mainブランチへPR作成
git checkout -b test/prod-deploy
git commit --allow-empty -m "test: prod deploy"
git push origin test/prod-deploy

gh pr create --base main --head test/prod-deploy
```

PR承認後、自動デプロイされます。

---

## ✅ 完了チェックリスト

- [ ] Wrangler ログイン済み
- [ ] Account ID 確認済み
- [ ] データベース作成済み（Staging & Production）
- [ ] Cloudflare Secrets 設定済み（全環境）
- [ ] データベースマイグレーション完了
- [ ] バックエンドデプロイ成功（Staging & Production）
- [ ] フロントエンドデプロイ成功（Staging & Production）
- [ ] GitHub Secrets 設定済み
- [ ] GitHub Environments 作成済み
- [ ] 自動デプロイ動作確認済み

---

## 🎉 完了！

### アクセスURL

**Staging:**
- Backend: https://pokedict-backend-staging.workers.dev
- Frontend: https://pokedict-frontend-staging.pages.dev

**Production:**
- Backend: https://pokedict-backend-prod.workers.dev
- Frontend: https://pokedict-frontend.pages.dev

---

## 📚 詳細ドキュメント

- [CLOUDFLARE_SETUP.md](./CLOUDFLARE_SETUP.md) - Cloudflare詳細設定
- [CLOUDFLARE_PAGES_SETUP.md](./CLOUDFLARE_PAGES_SETUP.md) - Pages設定
- [ENVIRONMENTS.md](./backend-hono/ENVIRONMENTS.md) - 環境構成
- [DEPLOYMENT.md](./backend-hono/DEPLOYMENT.md) - デプロイ詳細

---

## 🆘 トラブルシューティング

### デプロイエラー

```bash
# ログ確認
wrangler tail --env staging

# Secretsの確認
wrangler secret list --env staging
```

### データベース接続エラー

- 接続文字列が正しいか確認
- `?sslmode=require` が付いているか確認
- Neon/SupabaseのIPホワイトリスト確認

### 問題が解決しない

1. [CLOUDFLARE_SETUP.md](./CLOUDFLARE_SETUP.md) の詳細手順を確認
2. GitHub Issuesで報告
