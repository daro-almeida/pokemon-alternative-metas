use std::sync::Arc;

use axum::extract::FromRef;

use crate::application::services::arena::service::Arena;
#[derive(Clone)]
pub struct AppState {
    pub arena: Arc<Arena>,
}

impl AppState {
    pub fn new(arena: Arc<Arena>) -> Self {
        Self { arena }
    }
}

impl FromRef<AppState> for Arc<Arena> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.arena.clone()
    }
}
