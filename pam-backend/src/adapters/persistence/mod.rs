use sqlx::{Pool, Postgres};

use crate::application::AppError;

pub mod arena;

// #[derive(Debug, FromRow)]
// pub struct Run {
//     run_id: Uuid,
//     created_at: DateTime<Utc>,
//     finished: bool,
//     username: String,
// }

// #[derive(Debug, FromRow)]
// pub struct LeaderboardEntry {
//     meta: String,
//     username: String,
//     elo: i32,
//     gxe: i32,
//     glicko1: Glicko1,
//     coil: i32,
// }

// #[derive(Debug, sqlx::Type)]
// #[sqlx(type_name = "glicko1")]
// pub struct Glicko1 {
//     rating: i32,
//     deviation: i32,
// }

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