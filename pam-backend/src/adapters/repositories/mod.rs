use sqlx::{Pool, Postgres};

use crate::application::AppError;

pub mod arena;

#[derive(Clone)]
pub struct PostgresPersistence {
    pool: Pool<Postgres>,
}

impl PostgresPersistence {
    pub fn new(pool: Pool<Postgres>) -> Self {
        PostgresPersistence { pool }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::Database(value.to_string())
    }
}

impl From<time::error::Format> for AppError { 
    fn from(value: time::error::Format) -> Self {
        AppError::Database(value.to_string())
    }
}