use std::sync::Arc;

use axum::extract::FromRef;

use crate::application::services::{arena::service::Arena, matchmaking::Matchmaking};
#[derive(Clone)]
pub struct AppState {
    pub arena: Arc<Arena>,
    pub matchmaking: Arc<Matchmaking>,
}

impl AppState {
    pub fn new(arena: Arc<Arena>, matchmaking: Arc<Matchmaking>) -> Self {
        Self { arena, matchmaking }
    }
}

impl FromRef<AppState> for Arc<Arena> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.arena.clone()
    }
}

impl FromRef<AppState> for Arc<Matchmaking> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.matchmaking.clone()
    }
}
