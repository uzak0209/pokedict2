# 環境構成ガイド

このプロジェクトは3つの環境に分かれています。

## 📋 環境一覧

| 環境 | ブランチ | URL | 用途 |
|------|---------|-----|------|
| **Development** | `dev/*` | http://localhost:8081 | ローカル開発 |
| **Staging** | `staging`, `dev` | https://pokedict-backend-staging.workers.dev | テスト・QA |
| **Production** | `main` | https://pokedict-backend-prod.workers.dev | 本番環境 |

## 🔧 環境別設定

### Development (ローカル開発)

**使用するファイル:**
- `.env` (Node.js開発)
- `.dev.vars` (Wrangler開発)

**データベース:**
- ローカルPostgreSQL (`localhost:5432`)

**起動方法:**
```bash
# Node.js サーバー
npm run dev:node

# Wrangler エミュレーター
npm run dev
```

**環境変数:**
```env
DATABASE_URL=postgresql://postgres:password@localhost:5432/pokedict
JWT_SECRET=dev-secret-key-minimum-32-characters-long
ALLOWED_ORIGIN=http://localhost:5173
ENVIRONMENT=development
```

---

### Staging (ステージング環境)

**デプロイ対象ブランチ:**
- `staging` (推奨)
- `dev` (自動デプロイ)

**データベース:**
- 専用のStaging DB (Production DBとは完全分離)
- 推奨: Neon/Supabaseの別プロジェクト

**デプロイ方法:**
```bash
# 自動デプロイ
git push origin staging  # または dev

# 手動デプロイ
npm run deploy:staging
```

**環境変数設定:**
```bash
# Staging用のSecretsを設定
npx wrangler secret put DATABASE_URL --env staging
# 例: postgresql://user:pass@staging-db.region.neon.tech:5432/pokedict_staging

npx wrangler secret put JWT_SECRET --env staging
# 注意: Productionとは異なる値を使用

npx wrangler secret put ALLOWED_ORIGIN --env staging
# 例: https://staging.yourapp.com

npx wrangler secret put GEMINI_API_KEY --env staging
```

**用途:**
- 新機能のテスト
- QA環境
- クライアントへのデモ
- Production前の最終確認

**データ:**
- テストデータを使用
- 定期的にリセット可能
- Productionデータのコピーは避ける（個人情報保護）

---

### Production (本番環境)

**デプロイ対象ブランチ:**
- `main` のみ

**デプロイフロー:**
1. `staging`ブランチでテスト
2. Staging環境で動作確認
3. `main`ブランチへPR作成
4. レビュー後マージ
5. 自動的にProductionへデプロイ

**データベース:**
- 本番用DB (Stagingとは完全分離)
- バックアップ設定必須

**デプロイ方法:**
```bash
# 推奨: PR経由での自動デプロイ
# 1. staging -> main へPR作成
# 2. レビュー・承認
# 3. マージ → 自動デプロイ

# 緊急時の手動デプロイ
npm run deploy:production
```

**環境変数設定:**
```bash
# Production用のSecretsを設定
npx wrangler secret put DATABASE_URL --env production
# 例: postgresql://user:pass@prod-db.region.neon.tech:5432/pokedict

npx wrangler secret put JWT_SECRET --env production
# 重要: 強力なランダム文字列を使用（32文字以上）

npx wrangler secret put ALLOWED_ORIGIN --env production
# 例: https://yourapp.com

npx wrangler secret put GEMINI_API_KEY --env production
```

**セキュリティ要件:**
- JWT_SECRETは必ずStaging/Devと異なる値を使用
- DATABASE_URLは本番用DBのみを指定
- ALLOWED_ORIGINは本番フロントエンドのみ許可
- 全てのSecretsは定期的にローテーション

---

## 🔐 環境変数の管理

### 必須の環境変数

| 変数名 | 説明 | 環境ごとの違い |
|--------|------|----------------|
| `DATABASE_URL` | PostgreSQL接続文字列 | ✅ 環境ごとに異なるDB |
| `JWT_SECRET` | JWT署名用秘密鍵 | ✅ 環境ごとに異なる値 |
| `ALLOWED_ORIGIN` | CORS許可オリジン | ✅ 環境ごとに異なるURL |
| `GEMINI_API_KEY` | Gemini API キー | ⚠️ 同じでも可 |
| `ENVIRONMENT` | 環境名 | ✅ 自動設定（wrangler.toml） |

### Secretsの設定確認

```bash
# Staging
npx wrangler secret list --env staging

# Production
npx wrangler secret list --env production
```

### Secretsの更新

```bash
# 特定のSecretを更新
npx wrangler secret put JWT_SECRET --env production

# 削除（誤って設定した場合）
npx wrangler secret delete OLD_SECRET --env staging
```

---

## 🗄️ データベース構成

### 推奨構成

```
┌─────────────┐     ┌──────────────────┐
│ Development │ ──> │ Local PostgreSQL │
└─────────────┘     └──────────────────┘

┌─────────────┐     ┌──────────────────┐
│   Staging   │ ──> │ Neon Staging DB  │
└─────────────┘     └──────────────────┘

┌─────────────┐     ┌──────────────────┐
│ Production  │ ──> │ Neon Production  │
└─────────────┘     │ DB (with backup) │
                    └──────────────────┘
```

### データベースプロバイダー推奨

**Neon (推奨)**
- Serverless PostgreSQL
- 環境ごとに別プロジェクト作成
- 自動バックアップ
- 無料プランあり

**Supabase**
- PostgreSQL + 追加機能
- 環境ごとに別プロジェクト
- 無料プランあり

### マイグレーション管理

```bash
# 開発環境でマイグレーション作成
npm run db:generate

# Staging環境へマイグレーション適用
DATABASE_URL="postgresql://staging-db..." npm run db:migrate

# Production環境へマイグレーション適用
DATABASE_URL="postgresql://prod-db..." npm run db:migrate
```

---

## 🚀 デプロイフロー

### 通常の開発フロー

```
1. dev/* ブランチで開発
   ↓
2. dev ブランチへPR → マージ
   ↓ (自動デプロイ)
3. Staging環境でテスト
   ↓
4. staging ブランチへマージ（必要に応じて）
   ↓
5. main ブランチへPR → レビュー
   ↓ (承認後マージ → 自動デプロイ)
6. Production環境へリリース
```

### ホットフィックスフロー

```
1. hotfix/* ブランチ作成
   ↓
2. 修正 → staging ブランチへPR
   ↓ (自動デプロイ)
3. Staging環境で確認
   ↓
4. main ブランチへPR → マージ
   ↓ (自動デプロイ)
5. Production環境へ反映
```

---

## 📊 モニタリング

### ログ確認

```bash
# Staging環境のログ
npx wrangler tail --env staging

# Production環境のログ
npx wrangler tail --env production

# フィルタリング
npx wrangler tail --env production --status error
```

### メトリクス確認

Cloudflare Dashboard:
1. Workers & Pages > Overview
2. 環境ごとのWorkerを選択
3. Metrics タブ

---

## 🔒 セキュリティチェックリスト

### Staging環境

- [ ] Staging専用のデータベースを使用
- [ ] JWT_SECRETはDevelopment/Productionと異なる
- [ ] テストデータのみ使用（本番データは使わない）
- [ ] ALLOWED_ORIGINはStaging URLのみ許可
- [ ] アクセス制限（Basic認証等）を検討

### Production環境

- [ ] Production専用のデータベースを使用
- [ ] 強力なJWT_SECRET（32文字以上のランダム文字列）
- [ ] データベースバックアップが有効
- [ ] ALLOWED_ORIGINは本番URLのみ
- [ ] 全てのSecretsが正しく設定されている
- [ ] GitHub Environmentで承認フロー設定済み

---

## 🆘 トラブルシューティング

### 環境変数が反映されない

```bash
# Secretsを再設定
npx wrangler secret put DATABASE_URL --env production

# デプロイし直す
npm run deploy:production
```

### 誤った環境にデプロイしてしまった

```bash
# 前のバージョンにロールバック
npx wrangler rollback --env production

# または、正しいブランチから再デプロイ
```

### データベース接続エラー

1. DATABASE_URLが正しいか確認
2. データベースがCloudflare Workersからアクセス可能か確認
3. SSL接続設定を確認（`?sslmode=require`）
4. IPホワイトリストの確認（NeonやSupabaseの場合）

---

## 📚 関連ドキュメント

- [DEPLOYMENT.md](./DEPLOYMENT.md) - 詳細なデプロイ手順
- [README.md](./README.md) - プロジェクト概要
- [Cloudflare Workers Docs](https://developers.cloudflare.com/workers/)
