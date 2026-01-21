UPDATE teams
SET team_name = $2, description = $3, updated_at = NOW()
WHERE team_id = $1
