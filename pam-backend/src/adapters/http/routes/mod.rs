use axum::Router;

use crate::adapters::http::app_state::AppState;

pub mod arena;
pub mod matchmaking;

pub fn router() -> Router<AppState> {
    Router::new().nest("/arena", arena::router())
    //.nest("/matchmaking", matchmaking::router())
}
