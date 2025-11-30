use axum::{Router, routing::get};

use crate::metas::arena::picks::show_picks;



pub fn router() -> Router {
    Router::new()
        .route("/show_picks", get(show_picks))
}