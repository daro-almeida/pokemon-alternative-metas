use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;

use crate::{
    adapters::http::app_state::AppState,
    application::{AppResult, use_cases::arena::Arena},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{username}/run", get(show_run))
        .route("/{username}/options", get(show_options))
        .route("/{username}/pick", post(do_pick))
}

async fn show_run(
    State(arena): State<Arc<Arena>>,
    Path(username): Path<String>,
) -> AppResult<impl IntoResponse> {
    Ok(Json(arena.show_run(&username).await?))
}

async fn show_options(
    State(arena): State<Arc<Arena>>,
    Path(username): Path<String>,
) -> AppResult<impl IntoResponse> {
    Ok(Json(arena.show_options(&username).await?))
}

#[derive(Deserialize)]
pub struct ChoosePickRequest {
    option_no: usize,
}

async fn do_pick(
    State(arena): State<Arc<Arena>>,
    Path(username): Path<String>,
    Json(req): Json<ChoosePickRequest>,
) -> AppResult<impl IntoResponse> {
    Ok(Json(arena.do_pick(&username, req.option_no).await?))
}
