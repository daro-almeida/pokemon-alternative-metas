-- Add migration script here

ALTER TABLE arena_teams
ADD COLUMN bucket TEXT NOT NULL DEFAULT 'default_value';

ALTER TABLE arena_runs
ALTER COLUMN wins SET DEFAULT 0;

ALTER TABLE arena_runs
ALTER COLUMN losses SET DEFAULT 0;