use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use serde::Serialize;

use crate::{
    adapters::http::app_state::AppState,
    application::{AppResult, use_cases::arena::Arena},
    domain::pokemon::Pokemon,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/show_picks", get(show_current_run))
        .route("/pick", post(pick))
}

#[derive(Serialize)]
pub struct Pick {
    pick_num: u64,
    options: Vec<&'static Pokemon>,
}

async fn show_current_run(
    State(arena): State<Arc<Arena>>,
    Query(user): Query<String>,
) -> AppResult<Json<Pick>> {

    let pick = match get_user_current_run(&pool, &params.user).await? {
        Some(run_info) => {
            let options = get_user_options(&pool, &params.user).await?;
            if options.is_empty() {
                generate_pick(&pool, &run_info, &params.user).await?
            } else {
                Pick {
                    pick_num: run_info.pool.len() as u64 + 1,
                    options,
                }
            }
        }
        None => create_run(&pool, &params.user).await?,
    };

    Ok(Json(pick))
}

async fn pick(
    State(arena): State<Arc<Arena>>,
    Json((username, option)): Json<(String, i32)>,
) -> Result<Json<Pick>, StatusCode> {
    let pick = match get_user_current_run(&pool, &username).await {
        Some(run_info) => {
            let options = get_user_options(&pool, &username).await?;
            if options.is_empty() || options.len() <= option as usize {
                StatusCode::NOT_FOUND
            } else {
                commit_option(&pool, &run_info, &username, &options[option as usize]).await?;
                generate_pick(&pool, &run_info, &params.user).await?
            }
        }
        None => StatusCode::NOT_FOUND,
    }?;

    Ok(Json(pick))
}
