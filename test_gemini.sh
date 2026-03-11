#!/bin/bash
# Gemini API テストスクリプト

API_KEY=$(grep GEMINI_API_KEY .env | cut -d '=' -f2)

echo "Testing Gemini API directly..."
curl -X POST \
  "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=${API_KEY}" \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [{
      "parts": [{
        "text": "ポケモン「ピカチュウ」をチームに追加することを推奨する理由を50文字以内で簡潔に説明してください。"
      }]
    }]
  }' | python3 -m json.tool
