SELECT pokemon_id
FROM team_pokemon
WHERE team_id = $1
ORDER BY slot
