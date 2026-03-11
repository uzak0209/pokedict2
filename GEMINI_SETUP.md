# Gemini API 統合セットアップガイド

このプロジェクトでは、Google Gemini APIを使用してポケモンチーム提案に説得力のある説明を追加しています。

## 機能概要

- **既存のロジック**: タイプ相性や使用率データに基づいてポケモンを提案
- **Gemini AI**: 各提案について、なぜそのポケモンが推奨されるのかを日本語で説明
- **統合アプローチ**: LLMを使わずに提案を生成し、LLMは説明のみを補助

## セットアップ手順

### 1. Gemini API キーの取得

1. [Google AI Studio](https://makersuite.google.com/app/apikey) にアクセス
2. Googleアカウントでログイン
3. "Create API Key" をクリック
4. 生成されたAPIキーをコピー

### 2. 環境変数の設定

プロジェクトのルートディレクトリに `.env` ファイルを作成します：

```bash
# プロジェクトルートで実行
cp .env.example .env
```

`.env` ファイルを編集して、以下の値を設定：

```env
# Database Configuration
DATABASE_URL=postgresql://postgres:password@localhost:5432/pokedict

# JWT Configuration
JWT_SECRET=your-secret-key-here

# Gemini API Configuration
GEMINI_API_KEY=あなたのGemini APIキーをここに貼り付け
```

**重要**: `.env` ファイルは `.gitignore` に含まれているため、Gitにコミットされません。APIキーを安全に管理してください。

### 3. バックエンドのビルドと起動

```bash
cd backend
cargo build
USE_POSTGRES=true cargo run
```

### 4. フロントエンドの起動

```bash
cd frontend
npm install
npm run dev
```

## 使用方法

1. フロントエンドで「Team Builder」ページにアクセス
2. 4体までのポケモンを選択（チームの軸として）
3. "Generate Suggestions" ボタンをクリック
4. 提案されたポケモンのリストが表示され、各ポケモンの下に**LLMが生成した説明**が表示されます

例：
```
リザードン
├ スコア: 8
└ 説明: "ほのおタイプで高火力のアタッカー。みずタイプの弱点をカバーし、
         くさタイプに強い。ひこうタイプとの複合でじめん技を無効化できる。"
```

## API エンドポイント

### 新しいエンドポイント

**POST** `/api/pokemon/master/team/suggest-with-reasoning`

リクエスト:
```json
{
  "team": [25, 6, 9]  // form_idの配列
}
```

レスポンス:
```json
{
  "all_threats": [...],
  "axis_pokemon": [...],
  "suggestions": [
    {
      "form_id": 143,
      "name": "Snorlax",
      "name_ja": "カビゴン",
      "is_setup": false,
      "score": 12,
      "covered_threats": [...],
      "reasoning": "ノーマルタイプの耐久型。HPが非常に高く、特殊耐久に優れる。..."
    }
  ]
}
```

### 既存のエンドポイント（LLM説明なし）

**POST** `/api/pokemon/master/team/suggest`

LLM説明が不要な場合は、こちらのエンドポイントを使用できます。

## トラブルシューティング

### APIキーが無効

エラー: `"Gemini API error: HTTP 400: API key not valid"`

→ `.env` ファイルの `GEMINI_API_KEY` を確認してください

### LLM説明が表示されない

1. バックエンドのログを確認: `GEMINI_API_KEY not set, LLM features will be disabled`
2. `.env` ファイルが正しく読み込まれているか確認
3. バックエンドを再起動

### レート制限エラー

Gemini APIには無料枠のレート制限があります：
- 1分あたり15リクエスト
- 1日あたり1500リクエスト

レート制限に達した場合は、しばらく待ってから再試行してください。

## セキュリティ注意事項

⚠️ **重要**:
- APIキーを公開リポジトリにコミットしないでください
- `.env` ファイルは `.gitignore` に含まれています
- 本番環境では環境変数を安全に管理してください
- APIキーが漏洩した場合は、すぐにGoogle AI Studioで無効化して新しいキーを生成してください

## 開発モード

開発中にLLM機能を無効にしたい場合は、`GEMINI_API_KEY` を空にするか、`.env` ファイルから削除してください。その場合、`reasoning` フィールドは `null` になります。

## コスト

Google Gemini APIは無料枠があります：
- Gemini Pro: 1分あたり15リクエスト無料
- より高い制限が必要な場合は、Google Cloudの有料プランをご検討ください

詳細: https://ai.google.dev/pricing
