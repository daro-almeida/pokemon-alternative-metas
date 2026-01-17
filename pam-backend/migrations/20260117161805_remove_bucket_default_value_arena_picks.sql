-- Add migration script here

ALTER TABLE arena_picks
DROP COLUMN bucket;

ALTER TABLE arena_picks
ADD COLUMN bucket INT NOT NULL;