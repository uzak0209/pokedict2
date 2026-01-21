SELECT pokemon_id, user_id, form_id, nickname, level, nature, ability, item, tera_type,
       move1, move2, move3, move4,
       ev_hp, ev_attack, ev_defense, ev_sp_attack, ev_sp_defense, ev_speed,
       iv_hp, iv_attack, iv_defense, iv_sp_attack, iv_sp_defense, iv_speed,
       created_at
FROM user_pokemon
WHERE pokemon_id = $1
