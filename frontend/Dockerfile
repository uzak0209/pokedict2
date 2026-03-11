# ビルドステージ
FROM node:20-alpine as builder

WORKDIR /app

# 依存関係のインストール
COPY package*.json ./
RUN npm ci

# ソースコードをコピーしてビルド
COPY . .
RUN npm run build

# 実行ステージ（Nginx）
FROM nginx:alpine

# ビルドした成果物をコピー
COPY --from=builder /app/dist /usr/share/nginx/html

# Nginxの設定をコピー（SPA用）
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
