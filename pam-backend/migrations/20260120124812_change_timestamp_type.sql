-- Add migration script here
-- to timestamptz type

ALTER TABLE runs ALTER COLUMN created_at TYPE timestamptz;
ALTER TABLE runs ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;