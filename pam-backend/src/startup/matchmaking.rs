use std::sync::Arc;

use crate::application::{
    repositories::matchmaking::MatchmakingRepository, services::matchmaking::Matchmaking,
};

pub fn init_matchmaking_service(
    matchmaking_repository: Arc<dyn MatchmakingRepository>,
) -> anyhow::Result<Arc<Matchmaking>> {
    let service = Matchmaking::new(matchmaking_repository);
    service.start();
    Ok(Arc::new(service))
}
