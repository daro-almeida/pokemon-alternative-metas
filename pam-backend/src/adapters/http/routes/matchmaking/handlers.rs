use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{
        IntoResponse,
        sse::{Event, Sse},
    },
};
use futures::stream::Stream;
use std::{convert::Infallible, sync::Arc, time::Duration};
use uuid::Uuid;

use crate::application::{
    AppResult,
    services::{
        arena::service::Arena,
        matchmaking::{MatchResult, Matchmaking},
    },
};

pub(crate) async fn queue_match(
    State(matchmaking): State<Arc<Matchmaking>>,
    State(arena): State<Arc<Arena>>,
    Path(username): Path<String>,
) -> AppResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let run_info = arena.get_run_info(&username).await?;

    let mut rx = matchmaking.queue_for_match(run_info).await?;

    let stream = async_stream::stream! {
        while let Some(result) = rx.recv().await {
            match result {
                MatchResult::MatchFound(arena_match) => {
                    let json = serde_json::to_string(&arena_match).unwrap();
                    yield Ok(Event::default().event("match_found").data(json));
                    break;
                }
                MatchResult::SearchCancelled => {
                    yield Ok(Event::default().event("cancelled").data("Search cancelled"));
                    break;
                }
                MatchResult::Timeout => {
                    yield Ok(Event::default().event("timeout").data("No match found"));
                    break;
                }
            }
        }
    };

    Ok(Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(5))
            .text("keep-alive"),
    ))
}

pub(crate) async fn cancel_search(
    State(matchmaking): State<Arc<Matchmaking>>,
    Path(run_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let cancelled = matchmaking.cancel_search(&run_id);

    if cancelled {
        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
