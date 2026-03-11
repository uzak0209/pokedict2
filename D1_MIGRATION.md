# Cloudflare D1 移行ガイド

PostgreSQLからCloudflare D1（SQLite）への移行手順です。

## 🎯 D1とは

Cloudflare D1は、Cloudflare Workersと統合されたServerless SQLiteデータベースです。

**メリット:**
- ✅ Cloudflare Workersとネイティブ統合
- ✅ 追加の接続文字列不要（バインディングで自動接続）
- ✅ 無料枠が大きい
- ✅ グローバルレプリケーション対応
- ✅ 低レイテンシー

## ステップ1: D1データベースの作成

### Staging環境

```bash
wrangler d1 create pokedict-staging
```

出力例：
```
✅ Successfully created DB 'pokedict-staging' in region APAC
Created your database using D1's new storage backend. The new storage backend is not yet recommended for production workloads, but backs up your data via point-in-time restore.

[[d1_databases]]
binding = "DB" # i.e. available in your Worker on env.DB
database_name = "pokedict-staging"
database_id = "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
```

📝 **database_id** をコピーして保存

### Production環境

```bash
wrangler d1 create pokedict-production
```

📝 **database_id** をコピーして保存

---

## ステップ2: wrangler.toml の更新

`backend-hono/wrangler.toml` を編集：

```toml
# Staging Environment
[env.staging]
name = "pokedict-backend-staging"
vars = {
  ENVIRONMENT = "staging",
}

[[env.staging.d1_databases]]
binding = "DB"
database_name = "pokedict-staging"
database_id = "a1b2c3d4-e5f6-7890-abcd-ef1234567890" # ← ステップ1で取得したID

# Production Environment
[env.production]
name = "pokedict-backend-prod"
vars = {
  ENVIRONMENT = "production",
}

[[env.production.d1_databases]]
binding = "DB"
database_name = "pokedict-production"
database_id = "12345678-90ab-cdef-1234-567890abcdef" # ← ステップ1で取得したID
```

---

## ステップ3: スキーマ作成

### スキーマSQL作成

`backend-hono/drizzle/schema.sql` を作成：

```sql
-- Users table
CREATE TABLE users (
  user_id TEXT PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (unixepoch()),
  updated_at INTEGER NOT NULL DEFAULT (unixepoch())
);

-- Refresh tokens table
CREATE TABLE refresh_tokens (
  token_id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  token_hash TEXT NOT NULL UNIQUE,
  expires_at INTEGER NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (unixepoch()),
  revoked INTEGER NOT NULL DEFAULT 0,
  FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- User pokemon table
CREATE TABLE user_pokemon (
  pokemon_id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  form_id INTEGER NOT NULL,
  nickname TEXT,
  nature TEXT NOT NULL,
  ability TEXT NOT NULL,
  item TEXT,
  tera_type TEXT NOT NULL,
  move1 TEXT NOT NULL,
  move2 TEXT,
  move3 TEXT,
  move4 TEXT,
  ev_hp INTEGER NOT NULL DEFAULT 0,
  ev_attack INTEGER NOT NULL DEFAULT 0,
  ev_defense INTEGER NOT NULL DEFAULT 0,
  ev_special_attack INTEGER NOT NULL DEFAULT 0,
  ev_special_defense INTEGER NOT NULL DEFAULT 0,
  ev_speed INTEGER NOT NULL DEFAULT 0,
  iv_hp INTEGER NOT NULL DEFAULT 31,
  iv_attack INTEGER NOT NULL DEFAULT 31,
  iv_defense INTEGER NOT NULL DEFAULT 31,
  iv_special_attack INTEGER NOT NULL DEFAULT 31,
  iv_special_defense INTEGER NOT NULL DEFAULT 31,
  iv_speed INTEGER NOT NULL DEFAULT 31,
  created_at INTEGER NOT NULL DEFAULT (unixepoch()),
  FOREIGN KEY (user_id) REFERENCES users(user_id)
);

-- Teams table
CREATE TABLE teams (
  team_id TEXT PRIMARY KEY,
  owner_id TEXT NOT NULL,
  team_name TEXT NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (unixepoch()),
  updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
  FOREIGN KEY (owner_id) REFERENCES users(user_id)
);

-- Team pokemon table
CREATE TABLE team_pokemon (
  team_id TEXT NOT NULL,
  slot INTEGER NOT NULL,
  form_id INTEGER NOT NULL,
  terastal_type TEXT NOT NULL,
  FOREIGN KEY (team_id) REFERENCES teams(team_id)
);

-- Pokemon forms (master data)
CREATE TABLE pokemon_forms (
  form_id INTEGER PRIMARY KEY,
  species_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  name_jp TEXT NOT NULL,
  type1 TEXT NOT NULL,
  type2 TEXT
);
```

---

## ステップ4: マイグレーション実行

### Staging

```bash
cd backend-hono

# スキーマ適用
wrangler d1 execute pokedict-staging --file=./drizzle/schema.sql --env staging

# 確認
wrangler d1 execute pokedict-staging --command="SELECT name FROM sqlite_master WHERE type='table';" --env staging
```

### Production

```bash
wrangler d1 execute pokedict-production --file=./drizzle/schema.sql --env production

# 確認
wrangler d1 execute pokedict-production --command="SELECT name FROM sqlite_master WHERE type='table';" --env production
```

---

## ステップ5: コードの更新

必要なファイル変更（主要部分のみ）：

### 1. `src/db/connection.ts`

```typescript
import { drizzle } from 'drizzle-orm/d1';
import * as schema from './schema';

export function getDb(d1: D1Database) {
  return drizzle(d1, { schema });
}

export type DbType = ReturnType<typeof getDb>;
```

### 2. `src/index.ts`

```typescript
type Bindings = {
  DB: D1Database; // ← DATABASE_URLの代わりにD1Database
  JWT_SECRET: string;
  GEMINI_API_KEY?: string;
  ALLOWED_ORIGIN: string;
  ENVIRONMENT: string;
};

// 初期化
app.use('*', async (c, next) => {
  const db = getDb(c.env.DB); // ← D1バインディングを使用
  // ...
});
```

### 3. `drizzle.config.ts`

```typescript
import { defineConfig } from 'drizzle-kit';

export default defineConfig({
  schema: './src/db/schema.ts',
  out: './drizzle',
  dialect: 'sqlite',
  driver: 'd1-http',
});
```

---

## ステップ6: Secretsの更新

D1を使用すると、DATABASE_URLは不要になります：

### Staging

```bash
# JWT_SECRET
wrangler secret put JWT_SECRET --env staging

# ALLOWED_ORIGIN
wrangler secret put ALLOWED_ORIGIN --env staging

# GEMINI_API_KEY (オプション)
wrangler secret put GEMINI_API_KEY --env staging
```

### Production

```bash
wrangler secret put JWT_SECRET --env production
wrangler secret put ALLOWED_ORIGIN --env production
wrangler secret put GEMINI_API_KEY --env production
```

---

## ステップ7: ローカル開発

### ローカルD1の使用

```bash
cd backend-hono

# Wranglerの開発サーバー起動（ローカルD1を自動作成）
npm run dev
```

Wranglerが自動的にローカルSQLiteデータベースを`.wrangler/state/v3/d1/`に作成します。

### ローカルマイグレーション

```bash
# ローカルD1にスキーマ適用
wrangler d1 execute DB --local --file=./drizzle/schema.sql
```

---

## ステップ8: デプロイ

```bash
# Staging
npm run deploy:staging

# Production
npm run deploy:production
```

---

## 🔍 D1データの確認

### CLI経由

```bash
# Staging
wrangler d1 execute pokedict-staging --command="SELECT * FROM users LIMIT 10;" --env staging

# Production
wrangler d1 execute pokedict-production --command="SELECT * FROM users LIMIT 10;" --env production
```

### ダッシュボード

1. https://dash.cloudflare.com/ にログイン
2. **Workers & Pages** > **D1**
3. データベースを選択
4. **Console** タブでSQLを実行

---

## 📊 PostgreSQLとの違い

| 機能 | PostgreSQL | D1 (SQLite) |
|------|-----------|-------------|
| UUID型 | ✅ `uuid` | ❌ `TEXT` (文字列として保存) |
| TIMESTAMP | ✅ `timestamp` | ❌ `INTEGER` (unixepoch) |
| BOOLEAN | ✅ `boolean` | ❌ `INTEGER` (0/1) |
| 外部キー | ✅ 自動制約 | ✅ 対応（明示的にON必要） |
| トランザクション | ✅ 完全対応 | ✅ 対応 |
| 最大DBサイズ | 無制限 | 10GB (無料), 50GB (有料) |

---

## ✅ チェックリスト

- [ ] Staging D1データベース作成済み
- [ ] Production D1データベース作成済み
- [ ] wrangler.toml にdatabase_id設定済み
- [ ] スキーマSQL作成済み
- [ ] Stagingにマイグレーション適用済み
- [ ] Productionにマイグレーション適用済み
- [ ] コード更新済み（connection.ts, index.ts, drizzle.config.ts）
- [ ] Secretsを再設定済み（DATABASE_URLは削除）
- [ ] ローカル開発環境で動作確認済み
- [ ] Stagingデプロイ成功
- [ ] Productionデプロイ成功

---

## 🆘 トラブルシューティング

### D1が見つからない

**エラー**: `No D1 databases found`
- wrangler.tomlのdatabase_idを確認
- D1データベースが作成されているか確認: `wrangler d1 list`

### マイグレーションエラー

**エラー**: `table already exists`
- テーブルを削除してから再実行
- または、既存データをバックアップしてからdrop

### ローカル開発でD1が使えない

- `wrangler dev` を使用（`npm run dev:node` ではなく）
- ローカルD1は`.wrangler/state/`に自動作成される

---

## 🔗 参考リンク

- [Cloudflare D1 Docs](https://developers.cloudflare.com/d1/)
- [D1 Limits](https://developers.cloudflare.com/d1/platform/limits/)
- [Drizzle ORM with D1](https://orm.drizzle.team/docs/get-started-sqlite#cloudflare-d1)

