SELECT team_id, user_id, team_name, description, created_at, updated_at
FROM teams
WHERE team_id = $1
