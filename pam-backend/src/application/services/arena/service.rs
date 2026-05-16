use std::{collections::HashMap, sync::Arc};

use rand::seq::IteratorRandom;

use crate::{
    application::{
        AppError, AppResult, repositories::arena::ArenaRepository,
        services::arena::config::ArenaConfig,
    },
    domain::{
        arena::{pick::Pick, run::ArenaRunInfo},
        pokemon::Pokemon,
    },
};

pub struct Arena {
    config: ArenaConfig,
    arena_repository: Arc<dyn ArenaRepository>,
    bucket_pools: HashMap<usize, Vec<&'static Pokemon>>,
}

impl Arena {
    pub fn new(
        config: ArenaConfig,
        arena_repository: Arc<dyn ArenaRepository>,
        bucket_pools: HashMap<usize, Vec<&'static Pokemon>>,
    ) -> Self {
        Self {
            config,
            arena_repository,
            bucket_pools,
        }
    }

    pub async fn init(&self) -> AppResult<()> {
        self.arena_repository.delete_unfinished_draft_runs().await?;
        Ok(())
    }

    pub async fn show_run(&self, username: &str) -> AppResult<(ArenaRunInfo, Option<Pick>)> {
        let run_info = self.get_run_info(username).await?;
        let pick_opt = self.get_pick(&run_info).await?;

        Ok((run_info, pick_opt))
    }

    pub async fn abandon_run(&self, username: &str) -> AppResult<()> {
        if let Some(run_info) = self.arena_repository.get_user_current_run(username).await? {
            self.arena_repository.abandon_run(&run_info.run_id).await?;
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
            .arena_repository
            .pick_option(
                &run_info.run_id,
                option_no,
                run_info.team.len() + 1,
                self.config.num_picks,
            )
            .await?;

        run_info.team.push(pokemon);
        run_info.team_buckets.push(bucket);
        run_info.finished_draft = finished;

        self.get_pick(&run_info).await
    }

    async fn get_run_info(&self, username: &str) -> AppResult<ArenaRunInfo> {
        match self.arena_repository.get_user_current_run(username).await? {
            Some(run) => Ok(run),
            None => self.arena_repository.create_run(username).await,
        }
    }

    async fn get_pick(&self, run_info: &ArenaRunInfo) -> AppResult<Option<Pick>> {
        if run_info.finished_draft {
            return Ok(None);
        }

        if let Some((_, options)) = self
            .arena_repository
            .get_run_options(&run_info.run_id)
            .await?
        {
            Ok(Some(Pick {
                pick_num: run_info.team.len() + 1,
                options,
            }))
        } else {
            Ok(Some(self.generate_pick(run_info).await?))
        }
    }

    async fn generate_pick(&self, run_info: &ArenaRunInfo) -> AppResult<Pick> {
        fn include_in_pool(run_info: &ArenaRunInfo, pokemon: &'static Pokemon) -> bool {
            let all_diff_species = run_info.team.iter().all(|p| !pokemon.same_base_species(p));
            let only_one_mega = !pokemon.is_mega() || run_info.team.iter().all(|p| !p.is_mega());
            !run_info.team.contains(&pokemon) && all_diff_species && only_one_mega
        }

        let bucket_counts: HashMap<usize, usize> =
            run_info
                .team_buckets
                .iter()
                .fold(HashMap::new(), |mut map, bucket| {
                    *map.entry(*bucket).or_insert(0usize) += 1;
                    map
                });

        let bucket = self
            .config
            .quotas
            .iter()
            .enumerate()
            .flat_map(|(b, q)| std::iter::repeat_n(b, q - bucket_counts.get(&b).unwrap_or(&0usize)))
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

        self.arena_repository
            .insert_options(&run_info.run_id, bucket, &options)
            .await?;

        Ok(Pick {
            pick_num: run_info.team.len() + 1,
            options,
        })
    }
}
