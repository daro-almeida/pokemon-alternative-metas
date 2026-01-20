-- Add migration script here

ALTER TABLE leaderboards DROP COLUMN gxe;
ALTER TABLE leaderboards DROP COLUMN glicko1;
ALTER TABLE leaderboards DROP COLUMN coil;

-- drop glicko1 type
DROP TYPE glicko1;
