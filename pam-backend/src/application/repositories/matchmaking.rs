use async_trait::async_trait;
use uuid::Uuid;

use crate::{application::AppResult, domain::arena::{arena_match::ArenaMatch, run::ArenaRunInfo}};

#[async_trait]
pub trait MatchmakingRepository: Send + Sync {
    async fn get_matches(&self, username: &str) -> AppResult<Vec<ArenaMatch>>;
    async fn create_match(&self, run_info_1: &ArenaRunInfo, run_info_2: &ArenaRunInfo) -> AppResult<ArenaMatch>;
    async fn have_runs_played(&self, run_id_1: &Uuid, run_id_2: &Uuid) -> AppResult<bool>;
    async fn abandon_match(
        &self,
        match_id: &Uuid,
        username: &str,
        elo_change: i32,
    ) -> AppResult<()>;
}