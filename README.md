# ポケモン構築ビルダー (PokéDict2)

ポケモンSVの対面構築理論に基づいた、競技向けパーティ構築支援Webアプリケーション

## 🎯 プロジェクト概要

このプロジェクトは、ポケモン対戦における「強いパーティの組み方」を体系化し、
初心者から上級者まで使いやすいパーティ構築ツールを提供します。

### 対面構築理論について

[おしんのポケモンメモ置き場](https://oshinpoke.com/entry/2023/05/04/224904) の構築理論に基づき、
以下のバランスで組むことを推奨しています：

- **🛡️ 行動保証枠**: 3-4体（きあいのタスキ、ばけのかわなど）
- **⚔️ 崩し枠**: 1体（積み技 + 高火力）
- **🔄 クッション枠**: 1体（対面操作技持ち）
- **🎽 チョッキ枠**: 0-1体（特殊耐久重視）

## 🏗️ アーキテクチャ

```
pokedict2/
├── frontend/          # React + TypeScript + TailwindCSS
├── backend/           # Rust + Actix-web (Clean Architecture)
├── sync-pokeapi/      # PokeAPI同期ツール（未実装）
└── Makefile           # 開発コマンド集約
```

### フロントエンド
- **React 18** + **TypeScript**
- **Vite** (高速ビルドツール)
- **TailwindCSS v4** (モダンなスタイリング)

### バックエンド
- **Rust** + **Actix-web**
- **Clean Architecture** パターン
- Domain-driven design

## 🚀 クイックスタート

### 方法1: Docker を使う（推奨）

#### 前提条件
- **Docker** と **Docker Compose**

#### 開発環境の起動

```bash
# 開発環境を起動（ホットリロード対応）
make docker-dev
```

- フロントエンド: http://localhost:5173
- バックエンド: http://localhost:8080
- データベース: localhost:5432

#### 本番環境の起動

```bash
# イメージをビルド
make docker-build

# コンテナを起動
make docker-up

# ログを確認
make docker-logs
```

- アプリケーション: http://localhost:3000

#### コンテナの停止

```bash
make docker-down
```

### 方法2: ローカル環境で起動

#### 前提条件
- **Node.js** 18以上
- **Rust** 1.70以上
- **npm** または **yarn**

#### 1. セットアップ

```bash
# 依存関係をインストール
make install
```

#### 2. 開発サーバー起動

```bash
# フロントエンド + バックエンドを同時起動
make dev
```

- フロントエンド: http://localhost:5173
- バックエンド: http://localhost:8080

#### 3. 個別に起動する場合

```bash
# フロントエンドのみ
make dev-frontend

# バックエンドのみ
make dev-backend
```

## 📋 利用可能なコマンド

### セットアップ
```bash
make install           # 全依存関係をインストール
make install-frontend  # フロントエンドのみ
make install-backend   # バックエンドのみ
```

### 開発
```bash
make dev               # 開発サーバー起動（両方、ポート自動解放）
make dev-frontend      # フロントエンド開発サーバー（ポート5173自動解放）
make dev-backend       # バックエンド開発サーバー（ポート8080自動解放）
make kill-ports        # 使用中のポートを手動で解放（5173, 8080）
```

### ビルド
```bash
make build             # プロジェクト全体をビルド
make build-frontend    # フロントエンドのみ
make build-backend     # バックエンドのみ
```

### テスト
```bash
make test              # 全テスト実行
make test-backend      # バックエンドテストのみ
```

### Lint & Format
```bash
make format            # コードフォーマット
make format-check      # フォーマットチェック
make lint              # Lintチェック
```

### クリーンアップ
```bash
make clean             # ビルド成果物を削除
make clean-frontend    # フロントエンドのみ
make clean-backend     # バックエンドのみ
```

### Docker
```bash
# 本番モード
make docker-build      # イメージをビルド
make docker-up         # コンテナを起動（バックグラウンド）
make docker-down       # コンテナを停止
make docker-logs       # ログを表示
make docker-clean      # コンテナとボリュームを削除

# 開発モード
make docker-dev        # 開発環境を起動（ホットリロード）
make docker-dev-build  # 開発環境のイメージをビルド
make docker-logs-dev   # 開発環境のログを表示
```

### CI/CD
```bash
make ci                # CI用チェック（format, lint, test, build）
```

## 🎨 主な機能

### 実装済み
- ✅ パーティ構築UI（6スロット）
- ✅ 役割タグシステム（行動保証/崩し/クッション/チョッキ）
- ✅ パーティバランス表示
- ✅ 役割別推奨ポケモンリスト
- ✅ ポケモンタイプシステム（18タイプ対応）
- ✅ バックエンドドメインモデル（User, Team, Pokemon）
- ✅ タイプ相性計算システム

### 今後の実装予定
- [ ] ポケモン検索・選択モーダル
- [ ] バックエンドAPI実装
- [ ] タイプ相性自動分析
- [ ] パーティ保存/読み込み
- [ ] ユーザー認証
- [ ] PokeAPIとの統合
- [ ] ドラッグ&ドロップ並び替え
- [ ] パーティエクスポート/インポート

## 📁 プロジェクト構造

### フロントエンド (`frontend/`)
```
frontend/
├── src/
│   ├── components/        # Reactコンポーネント
│   │   ├── TeamBuilder.tsx
│   │   ├── PokemonSlot.tsx
│   │   ├── PartyBalanceIndicator.tsx
│   │   └── ...
│   ├── types/             # TypeScript型定義
│   ├── data/              # 定数データ
│   └── hooks/             # カスタムフック
└── package.json
```

### バックエンド (`backend/`)
```
backend/
├── src/
│   ├── domain/            # ドメイン層
│   │   ├── entity/        # エンティティ
│   │   │   ├── user.rs
│   │   │   ├── team.rs
│   │   │   └── pokemon_form.rs
│   │   └── valueobject/   # 値オブジェクト
│   │       ├── pokemontype.rs
│   │       └── typeset.rs
│   ├── handler/           # API ハンドラ（未実装）
│   ├── repository/        # リポジトリ（未実装）
│   └── usecase/           # ユースケース（未実装）
└── Cargo.toml
```

## 🧪 テスト

```bash
# バックエンドのテストを実行
make test-backend

# 詳細な出力
cd backend && cargo test --verbose
```

## 🔧 トラブルシューティング

### ポートが既に使用されている

開発サーバー起動時にポートエラーが出る場合：

```bash
# 自動解放（推奨）
make dev               # 起動時に自動でポートを解放

# 手動解放
make kill-ports        # ポート5173と8080を解放

# 特定のプロセスを確認
lsof -i:5173          # フロントエンドのポート
lsof -i:8080          # バックエンドのポート
```

**注意**: `make dev`、`make dev-frontend`、`make dev-backend` は起動前に自動的にポートを解放するため、通常は手動操作は不要です。

### その他のよくある問題

#### ビルドエラー

```bash
# キャッシュをクリアして再ビルド
make clean
make build
```

#### 依存関係のエラー

```bash
# 依存関係を再インストール
make install
```

## 🔧 開発環境

### VSCode 推奨拡張機能
- **Rust Analyzer** - Rustの言語サポート
- **ESLint** - JavaScriptのLint
- **Tailwind CSS IntelliSense** - TailwindのIntelliSense
- **Prettier** - コードフォーマッター

### コーディング規約
- Rust: `cargo fmt` + `cargo clippy` に準拠
- TypeScript: ESLint + Prettier に準拠

## 📝 ライセンス

MIT License

## 🙏 参考文献

- [おしんのポケモンメモ置き場](https://oshinpoke.com/)
- ポケモンの著作権は株式会社ポケモンに帰属します

## 🤝 コントリビューション

Issue や Pull Request を歓迎します！

---

**Made with ❤️ for Pokémon Trainers**
