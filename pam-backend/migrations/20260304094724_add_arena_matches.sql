-- Add migration script here

CREATE TABLE arena_matches (
    match_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    run_id_1 UUID NOT NULL REFERENCES runs(run_id) ON DELETE CASCADE,
    run_id_2 UUID NOT NULL REFERENCES runs(run_id) ON DELETE CASCADE,
    wins INT NOT NULL,
    losses INT NOT NULL,
    finished BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE arena_match_games(
    match_id UUID NOT NULL REFERENCES arena_matches(match_id) ON DELETE CASCADE,
    game_no INT NOT NULL,
    game_link TEXT NOT NULL,
    finished BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE arena_match_games ADD PRIMARY KEY (match_id, game_no);