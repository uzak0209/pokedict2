SELECT team_id, user_id, team_name, description, created_at, updated_at
FROM teams
WHERE user_id = $1
ORDER BY updated_at DESC
