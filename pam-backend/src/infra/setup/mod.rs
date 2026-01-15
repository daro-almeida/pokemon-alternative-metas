use std::sync::Arc;

use crate::{adapters::http::app_state::AppState, infra::{postgres_persistence, setup::{arena::load_arena, pokedex::POKEDEX}}};

pub mod arena;
pub mod pokedex;

pub async fn init_app_state() -> anyhow::Result<AppState> {
    let persistence = Arc::new(postgres_persistence().await?);

    let arena = load_arena(&POKEDEX, persistence.clone())?;

    Ok(AppState { arena: Arc::new(arena) })
}
