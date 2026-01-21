INSERT INTO teams (user_id, team_name, description, created_at, updated_at)
VALUES ($1, $2, $3, NOW(), NOW())
RETURNING team_id
