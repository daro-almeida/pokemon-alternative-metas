use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::application::repositories::pokemon::PokemonRepository;

pub mod arena_dao;
pub mod arena_repository;
pub mod matchmaking_repository;

pub struct PostgresArenaRepository {
    pool: Pool<Postgres>,
    pokemon_repository: Arc<dyn PokemonRepository>,
}

impl PostgresArenaRepository {
    pub fn new(pool: Pool<Postgres>, pokemon_repository: Arc<dyn PokemonRepository>) -> Self {
        Self {
            pool,
            pokemon_repository,
        }
    }
}

pub struct PostgresMatchmakingRepository {
    pool: Pool<Postgres>,
}

impl PostgresMatchmakingRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
