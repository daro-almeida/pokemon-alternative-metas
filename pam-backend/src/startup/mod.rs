use std::{fs::File, sync::Arc};

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::adapters::repositories::postgres::PostgresMatchmakingRepository;
use crate::startup::matchmaking::init_matchmaking_service;
use crate::{
    adapters::{
        http::app_state::AppState,
        repositories::{json::JsonPokemonRepository, postgres::PostgresArenaRepository},
    },
    startup::{arena::init_arena_service, database::init_pg_db},
};

pub mod app;
pub mod arena;
pub mod database;
pub mod matchmaking;
pub mod paths;

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let db_pool = init_pg_db().await?;

    let pokemon_repository = Arc::new(JsonPokemonRepository::new());
    let arena_repository = Arc::new(PostgresArenaRepository::new(
        db_pool.clone(),
        pokemon_repository.clone(),
    ));
    let arena = init_arena_service(pokemon_repository, arena_repository)?;
    let matchmaking_repository = Arc::new(PostgresMatchmakingRepository::new(db_pool));
    let matchmaking = init_matchmaking_service(matchmaking_repository)?;

    Ok(AppState::new(arena, matchmaking))
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // Add your crate name and all relevant modules
        "warn,axum_trainer=debug,tower_http=debug".into()
    });

    let console_layer = fmt::layer()
        .with_target(true) // Change to true to see where logs come from
        .with_level(true)
        .pretty();

    let file = File::create("app.log").expect("cannot create log file");
    let json_layer = fmt::layer()
        .json()
        .with_writer(file)
        .with_current_span(true)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(json_layer)
        .init();
}
