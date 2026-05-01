-- Add migration script here
DROP TABLE leaderboards;

CREATE TABLE arena_leaderboards (
    username TEXT NOT NULL,
    num_runs INTEGER NOT NULL,
    avg_wins REAL NOT NULL,
    PRIMARY KEY (username)
);