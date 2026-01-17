use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::{FromRow, types::time::PrimitiveDateTime};
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{AppError, AppResult, use_cases::arena::ArenaPersistence},
    domain::{
        arena::{ArenaRunInfo, Bucket},
        pokemon::Pokemon,
    },
};

#[derive(FromRow, Debug)]
pub struct ArenaRunInfoDb {
    pub run_id: Uuid,
    pub username: String,
    pub created_at: PrimitiveDateTime,
    pub wins: i32,
    pub losses: i32,
    pub finished_draft: bool,
    pub team: Vec<String>,
    pub team_buckets: Vec<i32>,
}

#[async_trait]
impl ArenaPersistence for PostgresPersistence {
    async fn delete_unfinished_draft_runs(&self) -> AppResult<()> {
        sqlx::query!(
            r#"
        DELETE FROM runs
        USING arena_runs
        WHERE runs.run_id = arena_runs.run_id
        AND arena_runs.finished_draft = false
        "#
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_user_current_run(
        &self,
        username: &str,
        pokedex: &'static HashMap<String, Pokemon>,
    ) -> AppResult<Option<ArenaRunInfo>> {
        let run_db = sqlx::query_as!(
            ArenaRunInfoDb,
            r#"
            SELECT 
                r.run_id,
                r.username,
                r.created_at,
                ar.wins,
                ar.losses,
                ar.finished_draft,
                COALESCE(ARRAY_AGG(at.pokemon ORDER BY at.pick_no) FILTER (WHERE at.pokemon IS NOT NULL), '{}') as "team!",
                COALESCE(ARRAY_AGG(at.bucket ORDER BY at.pick_no) FILTER (WHERE at.bucket IS NOT NULL), '{}') as "team_buckets!"
            FROM runs r
            INNER JOIN arena_runs ar ON r.run_id = ar.run_id
            LEFT JOIN arena_teams at ON r.run_id = at.run_id AND r.run_id = at.run_id
            WHERE r.username = $1 AND r.finished = false
            GROUP BY r.run_id, r.username, r.created_at, r.finished, ar.wins, ar.losses, ar.finished_draft
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)?;

        match run_db {
            Some(run_db) => {
                let team = run_db
                    .team
                    .iter()
                    .map(|poke_id| {
                        pokedex.get(poke_id).ok_or_else(|| {
                            AppError::Database(format!("{} not in pokedex", poke_id))
                        })
                    })
                    .collect::<AppResult<Vec<&'static Pokemon>>>()?;

                Ok(Some(ArenaRunInfo {
                    run_id: run_db.run_id,
                    created_at: run_db.created_at,
                    wins: run_db.wins as u32,
                    losses: run_db.losses as u32,
                    finished_draft: run_db.finished_draft,
                    team,
                    team_buckets: run_db.team_buckets.iter().map(|&b| b as Bucket).collect(),
                }))
            }
            None => Ok(None),
        }
    }

    async fn create_run(&self, username: &str) -> AppResult<ArenaRunInfo> {
        let mut tx = self.pool.begin().await?;

        let (run_id, created_at) = sqlx::query!(
            r#"
        INSERT INTO runs (username)
        VALUES ($1)
        RETURNING run_id, created_at
        "#,
            username
        )
        .fetch_one(&mut *tx)
        .await
        .map(|row| (row.run_id, row.created_at))?;

        sqlx::query!(
            r#"
        INSERT INTO arena_runs (run_id)
        VALUES ($1)
        "#,
            run_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(ArenaRunInfo {
            run_id,
            created_at,
            wins: 0,
            losses: 0,
            finished_draft: false,
            team: Vec::new(),
            team_buckets: Vec::new(),
        })
    }

    async fn insert_options(
        &self,
        run_id: &Uuid,
        bucket: Bucket,
        options: &[&'static Pokemon],
    ) -> AppResult<()> {
        let option_nos: Vec<i32> = (0..options.len()).map(|i| i as i32).collect();
        let ids = options
            .iter()
            .map(|p| p.id.to_owned())
            .collect::<Vec<String>>();

        sqlx::query!(
            r#"
        INSERT INTO arena_picks (run_id, bucket, option_no, pokemon)
        SELECT $1, $2, * FROM UNNEST($3::int[], $4::text[])
        "#,
            run_id,
            bucket as i32,
            &option_nos,
            &ids,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_run_options(
        &self,
        run_id: &Uuid,
        pokedex: &'static HashMap<String, Pokemon>,
    ) -> AppResult<Option<(Bucket, Vec<&'static Pokemon>)>> {
        let row = sqlx::query!(
            r#"
        SELECT 
            bucket,
            ARRAY_AGG(pokemon ORDER BY option_no) as "pokemon_ids!"
        FROM arena_picks
        WHERE run_id = $1
        GROUP BY bucket
        LIMIT 1
        "#,
            run_id
        )
        .fetch_optional(&self.pool)
        .await?;

        row.map(|row| {
            let bucket = row.bucket as Bucket;
            let pokemon_options = row
                .pokemon_ids
                .iter()
                .map(|poke_id| {
                    pokedex
                        .get(poke_id)
                        .ok_or_else(|| AppError::Database(format!("{} not in pokedex", poke_id)))
                })
                .collect::<AppResult<Vec<&'static Pokemon>>>()?;

            Ok((bucket, pokemon_options))
        })
        .transpose()
    }

    async fn pick_option(
        &self,
        run_id: &Uuid,
        option_no: usize,
        pick_no: usize,
        num_picks: usize,
    ) -> AppResult<()> {
        let mut tx = self.pool.begin().await?;

        let row = sqlx::query!(
            r#"
        WITH deleted AS (
            DELETE FROM arena_picks
            WHERE run_id = $1
            RETURNING run_id, bucket, pokemon, option_no
        )
        SELECT 
            bucket,
            ARRAY_AGG(pokemon ORDER BY option_no) as "pokemon_ids!"
        FROM deleted
        GROUP BY run_id, bucket
        LIMIT 1
        "#,
            run_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        let (bucket, options) = row
            .map(|row| (row.bucket as Bucket, row.pokemon_ids))
            .ok_or_else(|| AppError::NotFound("Pick not available".to_owned()))?;

        sqlx::query!(
            r#"
        INSERT INTO arena_teams (run_id, pick_no, pokemon, bucket)
        VALUES ($1, $2, $3, $4)
        "#,
            run_id,
            pick_no as i32,
            options[option_no],
            bucket as i32
        )
        .execute(&mut *tx)
        .await?;

        if pick_no == num_picks {
            sqlx::query!(
                r#"
            UPDATE arena_runs
            SET finished_draft = TRUE
            WHERE run_id = $1
            "#,
                run_id
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
