use std::{fs::File, sync::Arc};

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    adapters::http::app_state::AppState,
    infra::{
        postgres_persistence,
        setup::{arena::load_arena, pokedex::POKEDEX},
    },
};

pub mod arena;
pub mod pokedex;

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let persistence = Arc::new(postgres_persistence().await?);

    let arena = load_arena(&POKEDEX, persistence.clone())?;

    Ok(AppState {
        arena: Arc::new(arena),
    })
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            // Add your crate name and all relevant modules
            "warn,axum_trainer=debug,tower_http=debug".into()
        });
    
    let console_layer = fmt::layer()
        .with_target(true)  // Change to true to see where logs come from
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
