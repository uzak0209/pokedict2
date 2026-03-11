# Cloudflare 完全セットアップガイド

このガイドに従って、Cloudflare Workersへのデプロイ環境を構築します。

## 📋 前提条件

- [ ] Cloudflareアカウント（無料プランでOK）
- [ ] GitHubアカウント
- [ ] PostgreSQLデータベース（Neon推奨）

---

## ステップ1: Cloudflareアカウント設定

### 1-1. アカウント作成/ログイン

1. https://dash.cloudflare.com/ にアクセス
2. アカウントを作成（または既存アカウントでログイン）
3. 無料プランを選択

### 1-2. Account ID の取得

1. Cloudflare Dashboard にログイン
2. 右上のアカウント名をクリック
3. 任意のアカウントを選択
4. 右側の **Account ID** をコピーして保存

```
例: a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6
```

---

## ステップ2: Wrangler CLI のセットアップ

### 2-1. Wrangler へログイン

```bash
cd backend-hono
npx wrangler login
```

これでブラウザが開き、Cloudflareへの認証が行われます。

### 2-2. ログイン確認

```bash
npx wrangler whoami
```

以下のような出力が表示されればOK：

```
 ⛅️ wrangler 3.x.x
-------------------
Getting User settings...
👋 You are logged in with an OAuth Token, associated with the email 'your-email@example.com'!
┌───────────────────┬──────────────────────────────────┐
│ Account Name      │ Account ID                       │
├───────────────────┼──────────────────────────────────┤
│ Your Account      │ a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6 │
└───────────────────┴──────────────────────────────────┘
```

---

## ステップ3: API Token の作成（GitHub Actions用）

### 3-1. API Tokenを作成

1. Cloudflare Dashboard > **My Profile** > **API Tokens**
2. **Create Token** をクリック
3. **Edit Cloudflare Workers** テンプレートを選択
4. 設定を確認:
   - **Permissions**: `Account - Cloudflare Workers Scripts - Edit`
   - **Account Resources**: 該当のアカウントを選択
5. **Continue to summary** > **Create Token**
6. 表示されたトークンをコピーして**安全に保存**

⚠️ **重要**: このトークンは一度しか表示されません！

```
例: abcd1234efgh5678ijkl9012mnop3456qrst7890
```

---

## ステップ4: データベースのセットアップ

### 推奨: Neon (Serverless PostgreSQL)

#### 4-1. Neonアカウント作成

1. https://neon.tech/ にアクセス
2. GitHub アカウントでサインアップ（または通常サインアップ）
3. 無料プランを選択

#### 4-2. Staging用プロジェクト作成

1. **New Project** をクリック
2. 設定:
   - **Project name**: `pokedict-staging`
   - **PostgreSQL version**: 16（最新）
   - **Region**: Tokyo（または最寄りのリージョン）
3. **Create Project**

#### 4-3. Staging DB 接続文字列を取得

1. プロジェクトダッシュボードで **Connection Details** を表示
2. **Connection string** をコピー

```
postgresql://username:password@ep-xxx-xxx.region.aws.neon.tech/neondb?sslmode=require
```

#### 4-4. Production用プロジェクト作成

同じ手順で `pokedict-production` プロジェクトを作成

---

### 代替: Supabase

#### 4-1. Supabaseアカウント作成

1. https://supabase.com/ にアクセス
2. GitHub アカウントでサインアップ
3. 無料プランを選択

#### 4-2. プロジェクト作成

1. **New project** をクリック
2. Organization を選択（または作成）
3. 設定:
   - **Name**: `pokedict-staging`
   - **Database Password**: 強力なパスワード（保存必須）
   - **Region**: Tokyo（または最寄り）
4. **Create new project**

#### 4-3. 接続文字列を取得

1. **Project Settings** > **Database**
2. **Connection string** > **URI** をコピー

```
postgresql://postgres.xxx:[password]@xxx.pooler.supabase.com:5432/postgres
```

---

## ステップ5: Cloudflare Workers Secrets の設定

### 5-1. Staging環境のSecrets設定

```bash
cd backend-hono

# DATABASE_URL
npx wrangler secret put DATABASE_URL --env staging
# プロンプトが表示されたら、Staging DBの接続文字列を貼り付け

# JWT_SECRET (32文字以上のランダム文字列)
npx wrangler secret put JWT_SECRET --env staging
# 例: staging_jwt_secret_change_this_32chars_minimum

# ALLOWED_ORIGIN
npx wrangler secret put ALLOWED_ORIGIN --env staging
# 例: https://staging.yourapp.com (または http://localhost:5173 for testing)

# GEMINI_API_KEY (オプション)
npx wrangler secret put GEMINI_API_KEY --env staging
# Google AI Studio から取得したAPIキー
```

### 5-2. Production環境のSecrets設定

```bash
# DATABASE_URL
npx wrangler secret put DATABASE_URL --env production
# Production DBの接続文字列を貼り付け

# JWT_SECRET (Stagingとは異なる強力なシークレット)
npx wrangler secret put JWT_SECRET --env production
# 例: prod_jwt_super_secure_secret_32chars_or_more

# ALLOWED_ORIGIN
npx wrangler secret put ALLOWED_ORIGIN --env production
# 本番フロントエンドのURL（例: https://yourapp.com）

# GEMINI_API_KEY
npx wrangler secret put GEMINI_API_KEY --env production
```

### 5-3. Secrets確認

```bash
# Staging
npx wrangler secret list --env staging

# Production
npx wrangler secret list --env production
```

---

## ステップ6: GitHub Secrets の設定

### 6-1. GitHubリポジトリの設定画面へ

1. GitHubリポジトリページを開く
2. **Settings** > **Secrets and variables** > **Actions**

### 6-2. Repository secrets を追加

**New repository secret** をクリックして以下を追加:

#### CLOUDFLARE_API_TOKEN

- **Name**: `CLOUDFLARE_API_TOKEN`
- **Value**: ステップ3で作成したAPI Token

#### CLOUDFLARE_ACCOUNT_ID

- **Name**: `CLOUDFLARE_ACCOUNT_ID`
- **Value**: ステップ1-2で取得したAccount ID

### 6-3. 確認

2つのSecretsが表示されていればOK:
- ✅ CLOUDFLARE_API_TOKEN
- ✅ CLOUDFLARE_ACCOUNT_ID

---

## ステップ7: GitHub Environments の設定

### 7-1. Staging Environment 作成

1. **Settings** > **Environments**
2. **New environment** をクリック
3. 名前: `staging`
4. **Configure environment**
5. 設定:
   - **Deployment branches**: Add rule > `staging` と `dev` を追加
   - **Required reviewers**: 設定しない（自動デプロイ）

### 7-2. Production Environment 作成

1. **New environment** をクリック
2. 名前: `production`
3. **Configure environment**
4. 設定:
   - ✅ **Required reviewers**: チェックして2名以上を追加
   - **Deployment branches**: Add rule > `main` のみ
   - **Wait timer**: 必要に応じて設定（例: 5分）

---

## ステップ8: データベースマイグレーション

### 8-1. ローカルでマイグレーション生成

```bash
cd backend-hono
npm run db:generate
```

### 8-2. Staging DBへマイグレーション適用

```bash
# .envファイルにStaging DB URLを設定
echo "DATABASE_URL=postgresql://..." > .env

# マイグレーション実行
npm run db:migrate
```

### 8-3. Production DBへマイグレーション適用

```bash
# DATABASE_URLをProduction DBに変更
echo "DATABASE_URL=postgresql://..." > .env

# マイグレーション実行
npm run db:migrate
```

---

## ステップ9: テストデプロイ

### 9-1. Staging環境へ手動デプロイ

```bash
cd backend-hono
npm run deploy:staging
```

成功すると以下のようなURLが表示されます:
```
https://pokedict-backend-staging.workers.dev
```

### 9-2. 動作確認

```bash
# ヘルスチェック
curl https://pokedict-backend-staging.workers.dev/health

# 期待される出力:
# {"status":"ok","environment":"staging"}
```

### 9-3. Production環境へ手動デプロイ（オプション）

```bash
npm run deploy:production
```

---

## ステップ10: 自動デプロイのテスト

### 10-1. Staging自動デプロイテスト

```bash
# devブランチで変更してpush
git checkout dev
git commit --allow-empty -m "test: staging auto deploy"
git push origin dev
```

GitHub Actionsで自動デプロイが実行されます:
1. **Actions** タブで確認
2. `Deploy to Cloudflare Workers` ワークフロー
3. `Deploy to Staging` ジョブが成功すればOK

### 10-2. Production自動デプロイテスト

```bash
# mainブランチへPR作成
git checkout -b test/prod-deploy
git commit --allow-empty -m "test: production auto deploy"
git push origin test/prod-deploy

# GitHubでPR作成
gh pr create --base main --head test/prod-deploy

# マージ後、承認フローが動作
```

---

## ステップ11: カスタムドメイン設定（オプション）

### 11-1. Cloudflare Dashboardでドメイン追加

1. **Workers & Pages** > 該当のWorker
2. **Settings** > **Domains & Routes**
3. **Add Custom Domain**
4. ドメイン名を入力（例: `api.yourapp.com`）
5. DNS設定が自動で追加されます

### 11-2. フロントエンドの設定更新

```typescript
// frontend/.env
VITE_API_URL=https://api.yourapp.com
```

---

## ✅ セットアップ完了チェックリスト

### Cloudflare

- [ ] Cloudflareアカウント作成済み
- [ ] Account ID取得済み
- [ ] Wranglerでログイン済み
- [ ] API Token作成済み

### データベース

- [ ] Staging DB作成済み（Neon/Supabase）
- [ ] Production DB作成済み
- [ ] 両方の接続文字列取得済み
- [ ] マイグレーション適用済み

### Secrets

- [ ] Staging環境の全Secrets設定済み
- [ ] Production環境の全Secrets設定済み
- [ ] 各環境でJWT_SECRETが異なる値

### GitHub

- [ ] CLOUDFLARE_API_TOKEN設定済み
- [ ] CLOUDFLARE_ACCOUNT_ID設定済み
- [ ] Staging Environment作成済み
- [ ] Production Environment作成済み（承認者設定済み）

### デプロイ

- [ ] Staging環境へ手動デプロイ成功
- [ ] Production環境へ手動デプロイ成功
- [ ] Staging自動デプロイ成功
- [ ] Production承認フロー動作確認

---

## 🆘 トラブルシューティング

### デプロイが失敗する

**エラー**: `Authentication error`
- Wranglerで再ログイン: `npx wrangler login`
- GitHub SecretsのAPI Tokenを確認

**エラー**: `DATABASE_URL is not set`
- Cloudflare Secretsが正しく設定されているか確認
- `npx wrangler secret list --env staging`

### データベース接続エラー

**エラー**: `Connection timeout`
- データベースがCloudflare Workersからアクセス可能か確認
- Neon/SupabaseのIPホワイトリスト設定確認
- SSL接続が有効か確認（`?sslmode=require`）

### GitHub Actions失敗

- Secrets（CLOUDFLARE_API_TOKEN, CLOUDFLARE_ACCOUNT_ID）を確認
- wrangler.tomlの環境名が一致しているか確認
- Actionsのログを詳しく確認

---

## 📚 次のステップ

セットアップ完了後:

1. フロントエンドをCloudflare Pagesにデプロイ
2. カスタムドメインの設定
3. モニタリング・アラート設定
4. パフォーマンス最適化

---

## 🔗 関連リンク

- [Cloudflare Dashboard](https://dash.cloudflare.com/)
- [Neon Console](https://console.neon.tech/)
- [Supabase Dashboard](https://app.supabase.com/)
- [Cloudflare Workers Docs](https://developers.cloudflare.com/workers/)
- [Wrangler CLI Docs](https://developers.cloudflare.com/workers/wrangler/)
