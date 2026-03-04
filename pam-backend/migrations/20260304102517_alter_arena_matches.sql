-- Add migration script here

ALTER TABLE arena_matches DROP COLUMN wins;
ALTER TABLE arena_matches DROP COLUMN losses;

ALTER TABLE arena_matches ADD COLUMN run_1_wins INT NOT NULL DEFAULT 0;
ALTER TABLE arena_matches ADD COLUMN run_2_wins INT NOT NULL DEFAULT 0;
ALTER TABLE arena_matches ADD COLUMN winner_run_id UUID REFERENCES runs(run_id);

ALTER TABLE arena_matches ADD CONSTRAINT different_runs CHECK (run_id_1 != run_id_2);
ALTER TABLE arena_matches ADD CONSTRAINT valid_winner CHECK (
    (finished = FALSE AND winner_run_id IS NULL) OR
    (finished = TRUE AND winner_run_id IN (run_id_1, run_id_2))
);

ALTER TABLE arena_match_games DROP finished;
ALTER TABLE arena_match_games ADD winner_run_id UUID REFERENCES runs(run_id);