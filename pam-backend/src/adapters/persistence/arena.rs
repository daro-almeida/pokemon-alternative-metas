use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    adapters::persistence::PostgresPersistence,
    application::{AppError, AppResult, use_cases::arena::ArenaPersistence},
    domain::arena::ArenaRunInfo,
};

#[derive(Debug, FromRow)]
pub struct ArenaRun {
    run_id: Uuid,
    wins: i32,
    losses: i32,
}

#[derive(Debug, FromRow)]
pub struct ArenaTeam {
    run_id: Uuid,
    username: String,
    pick_no: i32,
    pokemon: String,
}

#[derive(Debug, FromRow)]
pub struct ArenaPick {
    username: String,
    option_no: i32,
    pokemon: String,
}

#[async_trait]
impl ArenaPersistence for PostgresPersistence {
    async fn get_user_current_run(&self, username: &str) -> AppResult<Option<ArenaRunInfo>> {
        Ok(sqlx::query_as!(
            ArenaRunInfo,
            r#"
            SELECT 
                r.run_id,
                r.username,
                ar.wins,
                ar.losses,
                COALESCE(ARRAY_AGG(at.pokemon ORDER BY at.pick_no) FILTER (WHERE at.pokemon IS NOT NULL), '{}') as "pool!",
                COALESCE(ARRAY_AGG(at.bucket ORDER BY at.pick_no) FILTER (WHERE at.bucket IS NOT NULL), '{}') as "pool_buckets!"
            FROM runs r
            INNER JOIN arena_runs ar ON r.run_id = ar.run_id
            LEFT JOIN arena_teams at ON r.run_id = at.run_id AND r.username = at.username
            WHERE r.username = $1 AND r.finished = false
            GROUP BY r.run_id, r.username, r.created_at, r.finished, ar.wins, ar.losses
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)?)
    }

    async fn create_run(pool: &Pool<Postgres>, username: &String) -> Result<Pick, DataError> {
        let mut tx = pool.begin().await?;

        let run_id = sqlx::query_scalar!(
            r#"
        INSERT INTO runs (username)
        VALUES ($1)
        RETURNING run_id
        "#,
            username
        )
        .fetch_one(&mut *tx)
        .await?;

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

        let run_info = ArenaRunInfoDb {
            run_id,
            username: username.clone(),
            wins: 0,
            losses: 0,
            pool: Vec::new(),
            pool_buckets: Vec::new(),
        };

        generate_pick(&pool, &run_info, username).await
    }

    async fn generate_pick(
        pool: &Pool<Postgres>,
        run_info: &ArenaRunInfoDb,
        username: &String,
    ) -> Result<Option<Pick>, DataError> {
        let mut tx: sqlx::Transaction<'_, Postgres> = pool.begin().await?;

        let bucket_counts: HashMap<usize, usize> =
            run_info
                .pool_buckets
                .iter()
                .fold(HashMap::new(), |mut map, bucket| {
                    *map.entry((*bucket).try_into().unwrap()).or_insert(0usize) += 1;
                    map
                });

        let bucket = ARENA_CONFIG
            .quotas
            .iter()
            .enumerate()
            .map(|(b, q)| std::iter::repeat_n(b, q - bucket_counts.get(&b).unwrap_or(&0usize)))
            .flatten()
            .choose(&mut rand::rng())
            .unwrap();

        let options = ARENA_POOL
            .get(&bucket)
            .unwrap()
            //.filter(|p| if run_info.pool.iter().any(|p2| ))
            .choose_multiple(&mut rand::rng(), ARENA_CONFIG.options_per_bucket[bucket])
            .cloned()
            .collect::<Vec<Pokemon>>();

        let option_nos: Vec<i32> = (0..options.len()).map(|i| i as i32).collect();
        let pokemon_ids: Vec<String> = options.iter().map(|p| p.id.clone()).collect();

        sqlx::query!(
            r#"
        INSERT INTO arena_picks (username, option_no, pokemon)
        SELECT $1, * FROM UNNEST($2::int[], $3::text[])
        "#,
            username,
            &option_nos,
            pokemon_ids.as_slice()
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(Pick {
            pick_num: run_info.pool.len() as u64 + 1,
            options,
        })
    }

    async fn get_user_options(
        pool: &Pool<Postgres>,
        username: &String,
    ) -> Result<Vec<Pokemon>, DataError> {
        Ok(sqlx::query_scalar!(
            r#"
        SELECT pokemon
        FROM arena_picks
        WHERE username = $1
        ORDER BY option_no
        "#,
            username
        )
        .fetch_all(pool)
        .await?
        .iter()
        .map(|s| Pokemon::try_new(s.clone()))
        .collect::<Result<Vec<Pokemon>, DataError>>()?)
    }
}
