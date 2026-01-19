# ビルドステージ
FROM rust:1.75 as builder

WORKDIR /app

# 依存関係のキャッシュ用に先にCargoファイルをコピー
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 実際のソースコードをコピーしてビルド
COPY src ./src
RUN touch src/main.rs && \
    cargo build --release

# 実行ステージ
FROM debian:bookworm-slim

WORKDIR /app

# 必要なランタイム依存関係をインストール
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# ビルドステージから実行ファイルをコピー
COPY --from=builder /app/target/release/backend /app/backend

# ポートを公開
EXPOSE 8080

# アプリケーションを実行
CMD ["/app/backend"]
