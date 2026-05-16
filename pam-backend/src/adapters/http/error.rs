use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::application::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self);
        match self {
            AppError::Database(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", error),
            )
                .into_response(),
            AppError::Internal(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal error: {}", error),
            )
                .into_response(),
            AppError::NotFound(error) => {
                (StatusCode::NOT_FOUND, format!("Not found: {}", error)).into_response()
            }
        }
    }
}
