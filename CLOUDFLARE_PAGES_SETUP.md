# Cloudflare Pages セットアップガイド

フロントエンドをCloudflare Pagesにデプロイする手順です。

## 📋 概要

Cloudflare Pagesは静的サイトホスティングサービスです。

### デプロイ環境

| 環境 | ブランチ | プロジェクト名 | バックエンドURL |
|------|---------|--------------|----------------|
| **Staging** | `staging`, `dev` | `pokedict-frontend-staging` | https://pokedict-backend-staging.workers.dev |
| **Production** | `main` | `pokedict-frontend` | https://pokedict-backend-prod.workers.dev |

---

## ステップ1: Cloudflare Pages プロジェクト作成

### 1-1. Staging プロジェクト作成

```bash
cd frontend

# Staging用にビルド
VITE_API_URL=https://pokedict-backend-staging.workers.dev npm run build

# Pages プロジェクト作成 & デプロイ
wrangler pages deploy dist --project-name=pokedict-frontend-staging
```

初回実行時、プロジェクトが自動作成されます。

### 1-2. Production プロジェクト作成

```bash
# Production用にビルド
VITE_API_URL=https://pokedict-backend-prod.workers.dev npm run build

# Pages プロジェクト作成 & デプロイ
wrangler pages deploy dist --project-name=pokedict-frontend
```

---

## ステップ2: GitHub Actions の設定

### 2-1. ワークフローファイル確認

`.github/workflows/deploy-frontend.yml` が作成されています。

このワークフローは以下を実行します：
- `dev`, `staging` ブランチ → Staging環境
- `main` ブランチ → Production環境

### 2-2. デプロイ確認

```bash
# devブランチへpush
git add .
git commit -m "feat: setup cloudflare pages"
git push origin dev
```

GitHub Actionsで自動デプロイが実行されます。

---

## ステップ3: 環境変数の設定

### 3-1. Cloudflare Dashboard で設定

1. [Cloudflare Dashboard](https://dash.cloudflare.com/) にログイン
2. **Workers & Pages** を選択
3. プロジェクトを選択（`pokedict-frontend-staging` または `pokedict-frontend`）
4. **Settings** > **Environment variables**

### 3-2. Staging環境変数

| 変数名 | 値 |
|--------|-----|
| `VITE_API_URL` | `https://pokedict-backend-staging.workers.dev` |
| `NODE_VERSION` | `20` |

### 3-3. Production環境変数

| 変数名 | 値 |
|--------|-----|
| `VITE_API_URL` | `https://pokedict-backend-prod.workers.dev` |
| `NODE_VERSION` | `20` |

---

## ステップ4: カスタムドメインの設定（オプション）

### 4-1. Staging ドメイン

1. Pages プロジェクト > **Custom domains**
2. **Set up a custom domain**
3. ドメイン名を入力（例: `staging.yourapp.com`）
4. DNS設定（自動または手動）

### 4-2. Production ドメイン

同様の手順で本番ドメインを設定（例: `yourapp.com` または `www.yourapp.com`）

---

## ステップ5: プレビューデプロイの設定

### 5-1. プレビュー環境

Cloudflare Pagesは自動的にPRごとにプレビュー環境を作成します。

- URL: `https://<commit-hash>.pokedict-frontend.pages.dev`

### 5-2. プレビュー環境変数

プレビュー環境用の変数も設定可能：

1. **Settings** > **Environment variables**
2. **Preview** タブで設定

---

## ステップ6: ビルド設定

### 6-1. Cloudflare Dashboard 設定

プロジェクト > **Settings** > **Builds & deployments**

**Build configuration:**
- **Framework preset**: `Vite`
- **Build command**: `npm run build`
- **Build output directory**: `dist`
- **Root directory**: `frontend`

### 6-2. Node.js バージョン

**Environment variables** で設定:
- `NODE_VERSION` = `20`

---

## ステップ7: CORS設定確認

バックエンド（Workers）のCORS設定を更新：

```typescript
// backend-hono/src/index.ts
cors({
  origin: [
    'http://localhost:5173',  // ローカル開発
    'https://pokedict-frontend-staging.pages.dev',  // Staging
    'https://pokedict-frontend.pages.dev',  // Production
    'https://yourapp.com',  // カスタムドメイン
  ],
  credentials: true,
})
```

---

## ステップ8: デプロイコマンド

### 手動デプロイ

```bash
cd frontend

# Staging
VITE_API_URL=https://pokedict-backend-staging.workers.dev npm run build
wrangler pages deploy dist --project-name=pokedict-frontend-staging

# Production
VITE_API_URL=https://pokedict-backend-prod.workers.dev npm run build
wrangler pages deploy dist --project-name=pokedict-frontend
```

### package.json に追加

```json
{
  "scripts": {
    "deploy:staging": "VITE_API_URL=https://pokedict-backend-staging.workers.dev npm run build && wrangler pages deploy dist --project-name=pokedict-frontend-staging",
    "deploy:production": "VITE_API_URL=https://pokedict-backend-prod.workers.dev npm run build && wrangler pages deploy dist --project-name=pokedict-frontend"
  }
}
```

---

## ステップ9: GitHub Environments 設定

### 9-1. Staging Frontend Environment

1. **Settings** > **Environments**
2. **New environment**: `staging-frontend`
3. 設定:
   - **Deployment branches**: `staging`, `dev`
   - **Required reviewers**: なし

### 9-2. Production Frontend Environment

1. **New environment**: `production-frontend`
2. 設定:
   - ✅ **Required reviewers**: 2名以上
   - **Deployment branches**: `main` のみ

---

## ステップ10: デプロイフロー

### 通常の開発フロー

```
1. feature/* ブランチで開発
   ↓
2. dev ブランチへマージ
   ↓ (自動デプロイ)
3. Staging環境でテスト
   - Backend: https://pokedict-backend-staging.workers.dev
   - Frontend: https://pokedict-frontend-staging.pages.dev
   ↓
4. main ブランチへPR
   ↓ (承認後、自動デプロイ)
5. Production環境へリリース
   - Backend: https://pokedict-backend-prod.workers.dev
   - Frontend: https://pokedict-frontend.pages.dev
```

---

## ✅ チェックリスト

### Cloudflare Pages

- [ ] Staging プロジェクト作成済み
- [ ] Production プロジェクト作成済み
- [ ] 環境変数設定済み（両環境）
- [ ] カスタムドメイン設定済み（オプション）

### GitHub

- [ ] deploy-frontend.yml 作成済み
- [ ] staging-frontend Environment 作成済み
- [ ] production-frontend Environment 作成済み（承認者設定）

### デプロイ

- [ ] Staging環境へ手動デプロイ成功
- [ ] Production環境へ手動デプロイ成功
- [ ] 自動デプロイ動作確認

### CORS

- [ ] バックエンドのCORS設定更新済み
- [ ] Staging環境で動作確認
- [ ] Production環境で動作確認

---

## 🔍 モニタリング

### デプロイ履歴確認

```bash
# Staging
wrangler pages deployment list --project-name=pokedict-frontend-staging

# Production
wrangler pages deployment list --project-name=pokedict-frontend
```

### ログ確認

Cloudflare Dashboard:
1. **Workers & Pages** > プロジェクト選択
2. **Deployments** タブで履歴確認
3. 各デプロイをクリックしてログ表示

---

## 🆘 トラブルシューティング

### ビルドが失敗する

**エラー**: `VITE_API_URL is not defined`
- 環境変数が正しく設定されているか確認
- ビルドコマンドで環境変数を指定

**エラー**: `Node version mismatch`
- `NODE_VERSION=20` を環境変数に設定

### デプロイが失敗する

**エラー**: `Project not found`
- プロジェクト名が正しいか確認
- 初回は自動作成されるが、プロジェクト名に制限あり

### CORS エラー

**エラー**: `Access to fetch blocked by CORS policy`
- バックエンドのCORS設定を確認
- `ALLOWED_ORIGIN` にフロントエンドURLを追加

---

## 📚 関連ドキュメント

- [Cloudflare Pages Docs](https://developers.cloudflare.com/pages/)
- [Wrangler Pages CLI](https://developers.cloudflare.com/pages/platform/deploy-hooks/)
- [Vite環境変数](https://vitejs.dev/guide/env-and-mode.html)

---

## 🎉 完了！

これでフロントエンドもCloudflare Pagesで自動デプロイされます！
