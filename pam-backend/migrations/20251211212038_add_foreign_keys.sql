-- Add migration script here

-- arena_runs.run_id references runs.run_id
ALTER TABLE arena_runs
ADD CONSTRAINT fk_arena_runs_run_id
FOREIGN KEY (run_id) REFERENCES runs(run_id) ON DELETE CASCADE;

-- arena_teams.run_id references runs.run_id
ALTER TABLE arena_teams
ADD CONSTRAINT fk_arena_teams_run_id
FOREIGN KEY (run_id) REFERENCES runs(run_id) ON DELETE CASCADE;