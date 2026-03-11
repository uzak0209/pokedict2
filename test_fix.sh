#!/bin/bash
# ユーザー登録
echo "Registering user..."
REGISTER_RES=$(curl -s -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser", "email": "test@example.com", "password": "password123"}')
echo "Register Response: $REGISTER_RES"

# ログイン
echo "\nLogging in..."
TOKEN=$(curl -s -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "password123"}' | sed -E 's/.*"access_token":"([^"]+)".*/\1/')

echo "\nToken: $TOKEN"

if [ "$TOKEN" = "null" ]; then
  echo "Login failed"
  exit 1
fi

# ポケモン登録 (ting-lu)
echo "\nRegistering Pokemon (ting-lu)..."
curl -v -X POST http://localhost:8080/api/pokemon \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "pokemon_name": "ting-lu",
    "pokemon_name_jp": "ディンルー",
    "nickname": "Test Ting-Lu",
    "nature": "Adamant",
    "ability": "Vessel of Ruin",
    "held_item": "Leftovers",
    "terastal_type": "Ground",
    "moves": ["Earthquake", "Heavy Slam", "Stealth Rock", "Whirlwind"],
    "ev_hp": 252,
    "ev_attack": 4,
    "ev_defense": 252,
    "ev_special_attack": 0,
    "ev_special_defense": 0,
    "ev_speed": 0,
    "iv_hp": 31,
    "iv_attack": 31,
    "iv_defense": 31,
    "iv_special_attack": 31,
    "iv_special_defense": 31,
    "iv_speed": 31
  }'
