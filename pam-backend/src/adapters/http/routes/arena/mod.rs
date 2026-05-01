use axum::{Router, routing::{get, post}};

use crate::adapters::http::app_state::AppState;

pub mod dto;
pub mod handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{username}/run", get(handlers::show_run))
        .route("/{username}/pick", post(handlers::do_pick))
        .route("/{username}/abandon", post(handlers::abandon_run))
}