use std::sync::Arc;

use axum::extract::FromRef;

use crate::application::use_cases::arena::Arena;

#[derive(Clone)]
pub struct AppState {
    pub arena: Arc<Arena>,
}

impl FromRef<AppState> for Arc<Arena> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.arena.clone()
    }
}