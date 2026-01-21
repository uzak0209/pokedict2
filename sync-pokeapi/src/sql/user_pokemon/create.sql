INSERT INTO user_pokemon (
    user_id, form_id, nickname, level, nature, ability, item, tera_type,
    move1, move2, move3, move4,
    ev_hp, ev_attack, ev_defense, ev_sp_attack, ev_sp_defense, ev_speed,
    iv_hp, iv_attack, iv_defense, iv_sp_attack, iv_sp_defense, iv_speed,
    created_at
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, NOW())
RETURNING pokemon_id
