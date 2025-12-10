-- Add migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE runs (
    run_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    finished BOOLEAN NOT NULL DEFAULT FALSE,
    username TEXT NOT NULL
);

CREATE TABLE arena_runs (
    run_id uuid PRIMARY KEY,
    wins INT NOT NULL,
    losses INT NOT NULL
);

CREATE TABLE arena_teams (
    run_id uuid NOT NULL,
    username TEXT NOT NULL,
    pick_no INT NOT NULL,
    pokemon TEXT NOT NULL,
    PRIMARY KEY (run_id, username, pick_no)
);

CREATE TABLE arena_picks (
    username TEXT NOT NULL,
    option_no INT NOT NULL,
    pokemon TEXT NOT NULL,
    PRIMARY KEY (username, option_no)
);

CREATE TYPE glicko1 AS (
    rating INT,
    deviation INT
);

CREATE TABLE leaderboards (
    meta TEXT NOT NULL,
    username TEXT NOT NULL,
    elo INT NOT NULL,
    gxe INT NOT NULL,
    glicko1 glicko1 NOT NULL,
    coil INT NOT NULL,
    PRIMARY KEY (meta, username)
);

