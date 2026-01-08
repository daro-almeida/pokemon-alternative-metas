-- Add migration script here

ALTER TABLE arena_teams
DROP COLUMN bucket;

ALTER TABLE arena_teams
ADD COLUMN bucket INT NOT NULL;