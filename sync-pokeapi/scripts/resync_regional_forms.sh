#!/bin/bash
# Re-sync all species with regional forms that were missing

DATABASE_URL="postgres://postgres:password@localhost:5432/pokedict"
SYNC_CMD="./target/release/sync-pokeapi --database-url $DATABASE_URL --smogon-only"

# Species IDs for Pokemon with missing regional forms
# Based on Smogon names that failed to map

# Alola forms
SPECIES_IDS=(
    38   # Ninetales (Alola)
    26   # Raichu (Alola)
    51   # Dugtrio (Alola)
    89   # Muk (Alola)
    76   # Golem (Alola)
    53   # Persian (Alola)
    103  # Exeggutor (Alola)
    27   # Sandslash (Alola) - actually Sandshrew evolves, check species
    28   # Sandslash (Alola)
    105  # Marowak (Alola)
)

# Hisui forms
SPECIES_IDS+=(
    503  # Samurott (Hisui)
    157  # Typhlosion (Hisui)
    549  # Lilligant (Hisui)
    724  # Decidueye (Hisui)
    706  # Goodra (Hisui)
    571  # Zoroark (Hisui)
    570  # Zorua (Hisui)
    101  # Electrode (Hisui)
    211  # Qwilfish (Hisui)
    713  # Avalugg (Hisui)
    901  # Ursaluna (has Bloodmoon form)
)

# Paldea forms (Tauros)
SPECIES_IDS+=(
    128  # Tauros (Paldea forms)
)

# Oricorio
SPECIES_IDS+=(
    741  # Oricorio
)

# Basculegion
SPECIES_IDS+=(
    902  # Basculegion
)

echo "Re-syncing species with regional forms..."

for id in "${SPECIES_IDS[@]}"; do
    echo "Syncing species $id..."
    ./target/release/sync-pokeapi --database-url "$DATABASE_URL" --species-id "$id" 2>&1 | grep -E "(Forms synced|Sync completed)"
done

echo "Done!"
