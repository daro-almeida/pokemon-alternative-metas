use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};

use crate::metas::arena::picks::show_picks;



pub fn router() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/show_picks", get(show_picks))
}