#!/bin/bash
set -e

# Missing regional forms IDs (Hisui, Alola, Origin forms etc.) that sometimes fail during full sync
IDS=(570 724 26 51 38 628 571 89 211 28 157 503 53 103 713 706 76 549 101 483 484 898)

echo "🔧 Repairing missing regional forms..."

for id in "${IDS[@]}"; do
    ./target/release/sync-pokeapi --database-url "$DATABASE_URL" --species-id $id
done

echo "✅ Repair completed."
