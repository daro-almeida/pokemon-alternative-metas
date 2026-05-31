use axum::{
    Router,
    routing::{get, post},
};

use crate::adapters::http::app_state::AppState;

pub mod dto;
pub mod handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{username}/queue", get(handlers::queue_match))
        .route("/{run_id}/cancel", post(handlers::cancel_search))
}
