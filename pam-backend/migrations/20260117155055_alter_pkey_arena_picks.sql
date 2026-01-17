-- Add migration script here

ALTER TABLE arena_picks DROP CONSTRAINT arena_picks_pkey;
ALTER TABLE arena_picks ADD PRIMARY KEY (run_id, option_no);