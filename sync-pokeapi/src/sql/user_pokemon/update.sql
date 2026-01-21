UPDATE user_pokemon
SET form_id = $2, nickname = $3, level = $4, nature = $5, ability = $6,
    item = $7, tera_type = $8, move1 = $9, move2 = $10, move3 = $11, move4 = $12,
    ev_hp = $13, ev_attack = $14, ev_defense = $15, ev_sp_attack = $16,
    ev_sp_defense = $17, ev_speed = $18,
    iv_hp = $19, iv_attack = $20, iv_defense = $21, iv_sp_attack = $22,
    iv_sp_defense = $23, iv_speed = $24
WHERE pokemon_id = $1
