
DELETE FROM arena_picks;

ALTER TABLE arena_picks ADD COLUMN run_id uuid;
ALTER TABLE arena_picks ADD CONSTRAINT arena_picks_run_id_fkey FOREIGN KEY (run_id) REFERENCES runs(run_id) ON DELETE CASCADE;
ALTER TABLE arena_picks DROP CONSTRAINT arena_picks_pkey;
ALTER TABLE arena_picks ADD PRIMARY KEY (run_id, username, option_no);
