use std::{collections::HashMap, ops::Not, sync::Arc};

use async_trait::async_trait;
use rand::seq::IteratorRandom;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    application::{AppError, AppResult},
    domain::{
        arena::{ArenaRunInfo, Bucket, Pick},
        pokemon::Pokemon,
    },
};

pub const META_STR: &str = "arena";

#[async_trait]
pub trait ArenaPersistence: Send + Sync {
    async fn delete_unfinished_draft_runs(&self) -> AppResult<()>;
    async fn create_run(&self, username: &str) -> AppResult<ArenaRunInfo>;
    async fn abandon_run(&self, run_id: &Uuid, username: &str, elo_change: i32) -> AppResult<()>;
    async fn get_user_current_run(
        &self,
        username: &str,
        pokedex: &'static HashMap<String, Pokemon>,
    ) -> AppResult<Option<ArenaRunInfo>>;
    async fn get_run_options(
        &self,
        run_id: &Uuid,
        pokedex: &'static HashMap<String, Pokemon>,
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
        pokedex: &'static HashMap<String, Pokemon>,
    ) -> AppResult<(bool, Bucket, &'static Pokemon)>;
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
    pokedex: &'static HashMap<String, Pokemon>,
    bucket_pools: HashMap<usize, Vec<&'static Pokemon>>,
    persistence: Arc<dyn ArenaPersistence>,
    config: ArenaConfig,
}

impl Arena {
    pub fn new(
        pokedex: &'static HashMap<String, Pokemon>,
        bucket_pools: HashMap<usize, Vec<&'static Pokemon>>,
        persistence: Arc<dyn ArenaPersistence>,
        config: ArenaConfig,
    ) -> Self {
        Self {
            pokedex,
            bucket_pools,
            persistence,
            config,
        }
    }

    pub async fn init(&self) -> AppResult<()> {
        self.persistence.delete_unfinished_draft_runs().await?;
        Ok(())
    }

    pub async fn show_run(&self, username: &str) -> AppResult<(ArenaRunInfo, Option<Pick>)> {
        let run_info = self.get_run_info(username).await?;
        let pick_opt = self.get_pick(&run_info).await?;

        Ok((run_info, pick_opt))
    }

    pub async fn abandon_run(&self, username: &str) -> AppResult<()> {
        if let Some(run_info) = self
            .persistence
            .get_user_current_run(username, self.pokedex)
            .await?
        {
            // TODO calc elos
            self.persistence.abandon_run(&run_info.run_id, username, 0).await?;
        }
        Ok(())
    }

    pub async fn do_pick(&self, username: &str, option_no: usize) -> AppResult<Option<Pick>> {
        let mut run_info = self.get_run_info(username).await?;
        if run_info.finished_draft {
            return Err(AppError::NotFound(
                "Draft finished. No picks available.".to_owned(),
            ));
        }

        let (finished, bucket, pokemon) = self
            .persistence
            .pick_option(
                &run_info.run_id,
                option_no,
                run_info.team.len() + 1,
                self.config.num_picks,
                self.pokedex,
            )
            .await?;

        run_info.team.push(pokemon);
        run_info.team_buckets.push(bucket);
        run_info.finished_draft = finished;

        self.get_pick(&run_info).await
    }

    async fn get_run_info(&self, username: &str) -> AppResult<ArenaRunInfo> {
        match self
            .persistence
            .get_user_current_run(username, self.pokedex)
            .await?
        {
            Some(run) => Ok(run),
            None => self.persistence.create_run(username).await,
        }
    }

    async fn get_pick(&self, run_info: &ArenaRunInfo) -> AppResult<Option<Pick>> {
        if run_info.finished_draft {
            return Ok(None);
        }

        if let Some((_, options)) = self
            .persistence
            .get_run_options(&run_info.run_id, self.pokedex)
            .await?
        {
            Ok(Some(Pick {
                pick_num: run_info.team.len() + 1,
                options,
            }))
        } else {
            Ok(Some(self.generate_pick(&run_info).await?))
        }
    }

    async fn generate_pick(&self, run_info: &ArenaRunInfo) -> AppResult<Pick> {
        fn include_in_pool(run_info: &ArenaRunInfo, pokemon: &'static Pokemon) -> bool {
            !run_info.team.contains(&pokemon)
                && (pokemon.base_species.is_none()
                    || run_info
                        .team
                        .iter()
                        .any(|p| pokemon.same_base_species(p))
                        .not())
                && (!pokemon.is_mega() || run_info.team.iter().any(|p| p.is_mega()).not())
        }

        let bucket_counts: HashMap<usize, usize> =
            run_info
                .team_buckets
                .iter()
                .fold(HashMap::new(), |mut map, bucket| {
                    *map.entry((*bucket).try_into().unwrap()).or_insert(0usize) += 1;
                    map
                });

        let bucket = self
            .config
            .quotas
            .iter()
            .enumerate()
            .map(|(b, q)| std::iter::repeat_n(b, q - bucket_counts.get(&b).unwrap_or(&0usize)))
            .flatten()
            .choose(&mut rand::rng())
            .ok_or_else(|| {
                AppError::Internal("No quotas left (should be unreachable)".to_owned())
            })?;

        let options = self
            .bucket_pools
            .get(&bucket)
            .ok_or_else(|| AppError::Internal(format!("No bucket {}", bucket)))?
            .iter()
            .filter(|p| include_in_pool(run_info, p))
            .copied()
            .choose_multiple(&mut rand::rng(), self.config.options_per_bucket[bucket]);

        self.persistence
            .insert_options(&run_info.run_id, bucket, &options)
            .await?;

        Ok(Pick {
            pick_num: run_info.team.len() + 1,
            options,
        })
    }
}
