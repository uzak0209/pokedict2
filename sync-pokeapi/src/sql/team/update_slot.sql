UPDATE team_pokemon
SET slot = $3
WHERE team_id = $1 AND pokemon_id = $2
