# GitHub Environment 設定ガイド

GitHub EnvironmentsをセットアップしてProduction環境へのデプロイに承認フローを追加します。

## 🎯 Environmentsの設定

### 1. Staging Environment

1. リポジトリの `Settings` > `Environments` へ移動
2. `New environment` をクリック
3. 名前: `staging` と入力
4. `Configure environment` をクリック

**設定:**
- ✅ **Deployment branches**: `staging` と `dev` ブランチのみ許可
- ❌ **Required reviewers**: なし（自動デプロイ）
- ❌ **Wait timer**: なし

**Environment secrets**: なし（Cloudflare Secretsを使用）

### 2. Production Environment

1. リポジトリの `Settings` > `Environments` へ移動
2. `New environment` をクリック
3. 名前: `production` と入力
4. `Configure environment` をクリック

**設定:**

#### ✅ Required reviewers
- チェックを入れる
- レビュワーを追加（最低1名、推奨2名）
- これにより、Productionデプロイ前に承認が必要になります

#### ✅ Deployment branches
- `Selected branches` を選択
- `main` ブランチのみ許可

#### ⚠️ Wait timer (オプション)
- 必要に応じて待機時間を設定
- 例: 5分間の待機時間を設定して、誤デプロイを防ぐ

**Environment secrets**: なし（Cloudflare Secretsを使用）

---

## 🔐 GitHub Secrets の設定

リポジトリの `Settings` > `Secrets and variables` > `Actions` で設定:

### Repository secrets

これらのSecretsはすべての環境で共有されます:

```
CLOUDFLARE_API_TOKEN
CLOUDFLARE_ACCOUNT_ID
```

#### CLOUDFLARE_API_TOKEN の取得方法

1. [Cloudflare Dashboard](https://dash.cloudflare.com/) にログイン
2. `My Profile` > `API Tokens` へ移動
3. `Create Token` をクリック
4. `Edit Cloudflare Workers` テンプレートを選択
5. トークンをコピーして、GitHub Secretsに追加

#### CLOUDFLARE_ACCOUNT_ID の取得方法

1. [Cloudflare Dashboard](https://dash.cloudflare.com/) にログイン
2. `Workers & Pages` > `Overview` へ移動
3. 右側に表示される `Account ID` をコピー

---

## 🚀 デプロイフローの動作

### Staging環境へのデプロイ

```
dev または staging ブランチへPush
  ↓
GitHub Actions が自動起動
  ↓
Lint & Type Check
  ↓
✅ 自動的にStaging環境へデプロイ
  ↓
https://pokedict-backend-staging.workers.dev へアクセス可能
```

**承認は不要です。**

### Production環境へのデプロイ

```
main ブランチへPush (通常はPRマージ)
  ↓
GitHub Actions が自動起動
  ↓
Lint & Type Check
  ↓
⏸️ デプロイ承認待ち (Required reviewers)
  ↓
承認者がレビュー・承認
  ↓
✅ Production環境へデプロイ
  ↓
https://pokedict-backend-prod.workers.dev へアクセス可能
```

**設定したレビュワーによる承認が必要です。**

---

## ✅ 承認フローの確認

### デプロイ承認の手順

1. `main`ブランチへPRをマージ
2. GitHub Actions が起動
3. `Deploy to Production` ジョブが待機状態に
4. GitHub から承認者へ通知
5. 承認者が以下を実施:
   - Actions タブへ移動
   - 待機中のワークフローをクリック
   - `Review deployments` をクリック
   - 変更内容を確認
   - `Approve and deploy` または `Reject` を選択

### 通知設定

承認待ちの通知を受け取るには:

1. GitHub プロフィール > `Settings` > `Notifications`
2. `Actions` の通知を有効化
3. メール通知を確認

---

## 📊 Environment の URL 設定

各Environmentに以下のURLを設定済み:

- **Staging**: https://pokedict-backend-staging.workers.dev
- **Production**: https://pokedict-backend-prod.workers.dev

これにより、PRやコミットの詳細からデプロイされた環境へ直接アクセスできます。

---

## 🔍 デプロイ履歴の確認

### GitHub UI

1. リポジトリの `Environments` タブをクリック
2. 環境を選択
3. デプロイ履歴を確認

### Cloudflare Dashboard

1. [Cloudflare Dashboard](https://dash.cloudflare.com/)
2. `Workers & Pages` > 該当のWorker
3. `Deployments` タブで履歴確認

---

## 🛡️ セキュリティのベストプラクティス

### ✅ 推奨設定

- [x] Production環境には必ず承認フローを設定
- [x] 最低2名のレビュワーを設定
- [x] `main`ブランチは保護されたブランチに設定
- [x] レビュワーは開発チームの複数メンバー

### ⚠️ 注意事項

- Cloudflare API Tokenは最小権限で作成
- Secretsは定期的にローテーション
- 退職者がレビュワーに含まれていないか定期確認

---

## 🆘 トラブルシューティング

### デプロイが承認待ちで止まる

**原因**: Production環境で承認フローが設定されている

**対処**:
1. Actions タブで承認
2. または、緊急時は手動デプロイ: `npm run deploy:production`

### 承認通知が来ない

**対処**:
1. GitHub通知設定を確認
2. メールフィルターを確認
3. Actions タブを定期的に確認

### ワークフローが失敗する

**確認項目**:
1. `CLOUDFLARE_API_TOKEN` が正しく設定されているか
2. `CLOUDFLARE_ACCOUNT_ID` が正しいか
3. Cloudflare Workers のクォータ制限に達していないか

---

## 📝 チェックリスト

設定完了後、以下を確認:

- [ ] Staging Environment 作成済み
- [ ] Production Environment 作成済み
- [ ] Production環境に承認フロー設定済み
- [ ] レビュワーが2名以上設定済み
- [ ] CLOUDFLARE_API_TOKEN を GitHub Secrets に設定済み
- [ ] CLOUDFLARE_ACCOUNT_ID を GitHub Secrets に設定済み
- [ ] `main` ブランチの保護ルール設定済み
- [ ] テストデプロイで動作確認済み

---

これで環境設定は完了です！🎉
