# Pokedict Backend (Hono)

RustからHonoに移行したバックエンドAPI - Cloudflare Workers対応

## セットアップ

### 1. 依存関係のインストール

```bash
npm install
```

### 2. 環境変数の設定

**Node.js開発の場合**

`.env.example`を`.env`にコピーして編集：

```bash
cp .env.example .env
```

**Wrangler開発の場合**

`.dev.vars.example`を`.dev.vars`にコピーして編集：

```bash
cp .dev.vars.example .dev.vars
```

必須の環境変数：
- `DATABASE_URL`: PostgreSQL接続文字列
- `JWT_SECRET`: JWT署名用秘密鍵（32文字以上）
- `ALLOWED_ORIGIN`: CORS許可オリジン

### 3. データベースのセットアップ

既存のPostgreSQLデータベースを使用します。

### 4. 開発サーバーの起動

**Node.js サーバー（従来の開発方法）**
```bash
npm run dev:node
```

**Wrangler開発サーバー（Cloudflare Workers エミュレーター）**
```bash
npm run dev
```

## スクリプト

### 開発
- `npm run dev` - Wrangler開発サーバー（Cloudflare Workers エミュレーター）
- `npm run dev:node` - Node.js開発サーバー（従来方式）
- `npm run build` - TypeScriptコンパイル
- `npm start` - Node.js本番サーバー起動

### デプロイ
- `npm run deploy` - Cloudflare Workersへデプロイ
- `npm run deploy:staging` - Staging環境へデプロイ
- `npm run deploy:production` - Production環境へデプロイ

### コード品質
- `npm run lint` - ESLintでコードチェック
- `npm run format` - Prettierでコード整形
- `npm run typecheck` - TypeScriptの型チェック

### データベース
- `npm run db:generate` - マイグレーション生成
- `npm run db:migrate` - マイグレーション実行
- `npm run db:studio` - Drizzle Studio起動

## API エンドポイント

### 認証

- `POST /api/auth/register` - ユーザー登録
- `POST /api/auth/login` - ログイン（リフレッシュトークンはHTTPOnly Cookieで返却）
- `POST /api/auth/refresh` - アクセストークンの更新
- `POST /api/auth/logout` - ログアウト

### 保護されたエンドポイント

`Authorization: Bearer <access_token>` ヘッダーが必要

- `GET /api/users/me` - 現在のユーザー情報取得

## アーキテクチャ

### ディレクトリ構造

```
src/
├── domain/           # ドメイン層
│   ├── entity/      # エンティティ
│   └── valueobject/ # 値オブジェクト
├── repository/       # リポジトリ層
│   ├── interface/   # リポジトリインターフェース
│   └── postgres/    # PostgreSQL実装
├── usecase/         # ユースケース層
├── handler/         # ハンドラー層
├── middleware/      # ミドルウェア
├── db/             # データベース設定
├── config/         # 設定
└── main.ts         # エントリーポイント
```

### 設計思想

- **ドメイン駆動設計**: エンティティと値オブジェクトでビジネスロジックを保護
- **依存性逆転**: リポジトリパターンでデータアクセス層を抽象化
- **型安全性**: TypeScript + Zodで実行時バリデーション
- **セキュリティ**:
  - JWT（アクセストークン + リフレッシュトークン）
  - リフレッシュトークンはHTTPOnly Cookieで保存
  - アクセストークンはレスポンスボディで返却
  - bcryptでパスワードハッシュ化

## CI/CD

### 自動デプロイ

GitHub Actionsで自動デプロイを実装：

| 環境 | トリガー | 承認 | URL |
|------|---------|------|-----|
| **Staging** | `staging`, `dev` ブランチへのPush | 不要 | https://pokedict-backend-staging.workers.dev |
| **Production** | `main` ブランチへのPush | **必要** | https://pokedict-backend-prod.workers.dev |

**詳細ドキュメント:**
- [DEPLOYMENT.md](./DEPLOYMENT.md) - デプロイ手順
- [ENVIRONMENTS.md](./ENVIRONMENTS.md) - 環境構成の詳細
- [.github/ENVIRONMENT_SETUP.md](./.github/ENVIRONMENT_SETUP.md) - GitHub環境設定

### Git Hooks (Husky)

- **pre-commit**: ESLint + Prettier でコード品質チェック
- **pre-push**: TypeScript型チェック

## デプロイ環境

- **プラットフォーム**: Cloudflare Workers
- **データベース**: 外部PostgreSQL（Neon、Supabase等を推奨）
- **CI/CD**: GitHub Actions + Wrangler
