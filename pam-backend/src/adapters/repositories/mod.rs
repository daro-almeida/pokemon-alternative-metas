use crate::application::AppError;

pub mod json;
pub mod postgres;

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::Database(value.to_string())
    }
}
