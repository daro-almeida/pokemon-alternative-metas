-- Add migration script here

-- change pkey to (run_id, pick_no)
ALTER TABLE arena_teams
DROP CONSTRAINT arena_teams_pkey;

ALTER TABLE arena_teams
ADD PRIMARY KEY (run_id, pick_no);

ALTER TABLE arena_teams
DROP COLUMN username;