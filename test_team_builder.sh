#!/bin/bash

echo "=== Team Builder Feature Test Script ==="
echo ""

# Test 1: Team Suggestion API (without auth)
echo "Test 1: Team Suggestion API (anonymous)"
curl -s -X POST "http://localhost:8080/api/pokemon/master/team/suggest" \
  -H "Content-Type: application/json" \
  -d '{"team": [1003, 10194, 1007, 987]}' | python3 -c "import sys, json; data=json.load(sys.stdin); print(f'✓ Got {len(data[\"suggestions\"])} suggestions, {len(data[\"uncovered_threats\"])} uncovered threats')"

echo ""

# Test 2: Matchup Matrix API (without auth)
echo "Test 2: Matchup Matrix API (anonymous)"
curl -s "http://localhost:8080/api/pokemon/master/matrix?limit=10" | python3 -c "import sys, json; data=json.load(sys.stdin); print(f'✓ Got matrix with {len(data[\"pokemon\"])} pokemon, {len(data[\"cells\"])} cells')"

echo ""

# Test 3: Top Pokemon API
echo "Test 3: Top Pokemon API"
curl -s "http://localhost:8080/api/pokemon/master/top?limit=5" | python3 -c "import sys, json; data=json.load(sys.stdin); print(f'✓ Got {data[\"total\"]} pokemon')"

echo ""

# Test 4: Check database table structure
echo "Test 4: Database table structure"
docker exec pokedict-db-dev psql -U postgres -d pokedict -c "\d user_matchup_overrides" 2>&1 | grep -q "judgment" && echo "✓ user_matchup_overrides table has correct structure" || echo "✗ Table structure issue"

echo ""
echo "=== All tests completed ==="
echo ""
echo "To test matchup override (requires authentication):"
echo "1. Login to the app"
echo "2. Go to Team Builder"
echo "3. Select 4 axis Pokemon"
echo "4. Click 'Generate Suggestions'"
echo "5. Click on any cell in the coverage matrix"
echo "6. The cell should toggle between ○ and -"
