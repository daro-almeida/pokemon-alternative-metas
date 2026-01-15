use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    application::AppResult, domain::{ArenaRunInfo, pokemon::Pokemon},
};

#[async_trait]
pub trait ArenaPersistence: Send + Sync {
    async fn get_user_current_run(&self, username: &str) -> AppResult<Option<ArenaRunInfo>>;
}

#[derive(Deserialize)]
pub struct ArenaConfig {
    pub num_picks: usize,
    pub num_buckets: usize,
    pub points_to_bucket: HashMap<usize, usize>,
    pub options_per_bucket: Vec<usize>,
    pub quotas: Vec<usize>,
}

pub struct Arena {
    pool: HashMap<usize, Vec<&'static Pokemon>>,
    persistence: Arc<dyn ArenaPersistence>,
    config: ArenaConfig,
}

impl Arena {
    pub fn new(
        pool: HashMap<usize, Vec<&'static Pokemon>>,
        persistence: Arc<dyn ArenaPersistence>,
        config: ArenaConfig,
    ) -> Self {
        Self {
            pool,
            persistence,
            config,
        }
    }

    pub async fn show_picks(&self, username: &str) -> AppResult<Pick> {
    }
}
