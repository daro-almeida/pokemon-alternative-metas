use std::sync::Arc;

use axum::{Json, extract::{Path, State}, response::IntoResponse};

use crate::{adapters::http::routes::arena::dto::ChoosePickRequest, application::{AppResult, services::arena::service::Arena}};

pub(super) async fn show_run(
    State(arena): State<Arc<Arena>>,
    Path(username): Path<String>,
) -> AppResult<impl IntoResponse> {
    Ok(Json(arena.show_run(&username).await?))
}

pub(super) async fn do_pick(
    State(arena): State<Arc<Arena>>,
    Path(username): Path<String>,
    Json(req): Json<ChoosePickRequest>,
) -> AppResult<impl IntoResponse> {
    Ok(Json(arena.do_pick(&username, req.option_no).await?))
}

pub(super) async fn abandon_run(
    State(arena): State<Arc<Arena>>,
    Path(username): Path<String>,
) -> AppResult<impl IntoResponse> {
    arena.abandon_run(&username).await?;
    Ok(())
}