use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    application::AppResult,
    domain::{
        arena::run::{ArenaRunInfo, Bucket},
        pokemon::Pokemon,
    },
};

#[async_trait]
pub trait ArenaRepository: Send + Sync {
    async fn delete_unfinished_draft_runs(&self) -> AppResult<()>;
    async fn create_run(&self, username: &str) -> AppResult<ArenaRunInfo>;
    async fn abandon_run(&self, run_id: &Uuid) -> AppResult<()>;
    async fn get_user_current_run(&self, username: &str) -> AppResult<Option<ArenaRunInfo>>;
    async fn get_run_options(
        &self,
        run_id: &Uuid,
    ) -> AppResult<Option<(Bucket, Vec<&'static Pokemon>)>>;
    async fn insert_options(
        &self,
        run_id: &Uuid,
        bucket: usize,
        options: &[&'static Pokemon],
    ) -> AppResult<()>;
    async fn pick_option(
        &self,
        run_id: &Uuid,
        option_no: usize,
        pick_no: usize,
        num_picks: usize,
    ) -> AppResult<(bool, Bucket, &'static Pokemon)>;
}
