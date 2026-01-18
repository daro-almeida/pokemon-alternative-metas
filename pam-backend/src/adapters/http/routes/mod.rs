use axum::{Router, http::StatusCode, response::{IntoResponse, Response}};

use crate::{adapters::http::app_state::AppState, application::AppError};

pub mod arena;

pub fn router() -> Router<AppState> {
    Router::new().nest("/arena", arena::router())
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self);
        match self {
            AppError::Database(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", error)).into_response()
            }
            AppError::Internal(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal error: {}", error)).into_response()
            }
            AppError::NotFound(error) => {
                (StatusCode::NOT_FOUND, format!("Not found: {}", error)).into_response()
            },
        }
    }
}