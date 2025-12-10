use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use rand::{self, Rng, seq::IndexedRandom};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

use crate::data::{DataError, Pokemon};

fn load_arena_config() -> Result<ArenaConfig, DataError> {
    let config = read_to_json("src/data/arena/config.json")?;

    let points_to_bucket = config["points_to_bucket"]
        .as_object()
        .ok_or(DataError::Format("points_to_bucket".into()))?
        .iter()
        .map(|(k, v)| {
            let num = k
                .parse::<usize>()
                .map_err(|_| DataError::Format(format!("points_to_bucket: k: {}", k)))?;
            let bucket = v
                .as_u64()
                .ok_or(DataError::Format(format!("points_to_bucket v: {}", v)))?;
            Ok((num, bucket as usize))
        })
        .collect::<Result<HashMap<usize, usize>, DataError>>()?;

    let quotas = config["quotas"]
        .as_array()
        .ok_or(DataError::Format("quotas".into()))?
        .iter()
        .map(|quota| {
            Ok(quota
                .as_u64()
                .ok_or(DataError::Format(format!("{}", quota)))? as usize)
        })
        .collect::<Result<Vec<usize>, DataError>>()?;

    let num_picks = quotas.iter().sum();
    let num_buckets = quotas.len();

    let options_per_bucket = config["options_per_bucket"]
        .as_array()
        .ok_or(DataError::Format("options_per_bucket".into()))?
        .iter()
        .map(|num_options| {
            Ok(num_options
                .as_u64()
                .ok_or(DataError::Format(format!("{}", num_options)))? as usize)
        })
        .collect::<Result<Vec<usize>, DataError>>()?;

    Ok(ArenaConfig {
        points_to_bucket,
        options_per_bucket,
        quotas,
        num_picks,
        num_buckets,
    })
}

fn load_arena_pool(config: &ArenaConfig) -> Result<HashMap<usize, Vec<Pokemon>>, DataError> {
    let mut points_pool = read_to_json("src/data/arena/draft_points.json")?
        .as_object()
        .ok_or(DataError::Format("arena/draft_points.json".into()))?
        .iter()
        .map(|(k, v)| {
            let num = k
                .parse::<usize>()
                .map_err(|_| DataError::Format(format!("{}", k)))?;

            let pokemons: Vec<Pokemon> = v
                .as_array()
                .ok_or(DataError::Format(format!("{}", k)))?
                .iter()
                .map(|pid| {
                    Pokemon::try_new(
                        pid.as_str()
                            .ok_or(DataError::Format(format!("{}, {}", k, pid)))?
                            .into(),
                    )
                })
                .collect::<Result<Vec<Pokemon>, DataError>>()?;

            Ok((num, pokemons))
        })
        .collect::<Result<HashMap<usize, Vec<Pokemon>>, DataError>>()?;

    let exceptions: Vec<String> = read_to_json("src/data/arena/exceptions.json")?
        .as_array()
        .ok_or(DataError::Format("arena/exceptions.json".into()))?
        .iter()
        .map(|pid| {
            Ok(pid
                .as_str()
                .ok_or(DataError::Format(pid.to_string()))?
                .into())
        })
        .collect::<Result<Vec<String>, DataError>>()?;

    points_pool
        .get_mut(&1)
        .ok_or(DataError::Format("point 1 list missing".into()))?
        .retain(|p| !p.has_evo() || exceptions.contains(&p.id));

    let pool = points_pool.into_iter().try_fold(
        HashMap::new(),
        |mut acc: HashMap<usize, Vec<Pokemon>>, (points, pokes)| {
            let bucket = *config
                .points_to_bucket
                .get(&points)
                .ok_or(DataError::Format(format!("{} not in config", points)))?;

            acc.entry(bucket).or_default().extend(pokes);
            Ok::<_, DataError>(acc)
        },
    )?;

    for i in 0..pool.len() {
        dbg!(i, pool[&i].len());
    }

    Ok(pool)
}

struct ArenaConfig {
    points_to_bucket: HashMap<usize, usize>,
    options_per_bucket: Vec<usize>,
    quotas: Vec<usize>,
    num_picks: usize,
    num_buckets: usize,
}

static ARENA_CONFIG: Lazy<ArenaConfig> = Lazy::new(|| load_arena_config().unwrap());

static ARENA_POOL: Lazy<HashMap<usize, Vec<Pokemon>>> =
    Lazy::new(|| load_arena_pool(&ARENA_CONFIG).unwrap());

fn read_to_json(path: &str) -> Result<Value, DataError> {
    let file = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&file)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct RunInfo {
    username: String,
    //created_at: DateTime<Utc>,
    wins: i32,
    losses: i32,
    pool: Vec<String>,
}

#[derive(Serialize)]
pub struct Pick {
    pick_num: u64,
    options: Vec<&'static Pokemon>,
}

#[derive(Deserialize)]
pub struct ShowPicksParams {
    user: String,
}

async fn get_user_current_run(
    pool: &Pool<Postgres>,
    username: &String,
) -> Result<Option<RunInfo>, sqlx::Error> {
    Ok(sqlx::query_as!(
        RunInfo,
        r#"
        SELECT 
            r.username,
            ar.wins,
            ar.losses,
            COALESCE(ARRAY_AGG(at.pokemon ORDER BY at.pick_no) FILTER (WHERE at.pokemon IS NOT NULL), '{}') as "pool!"
        FROM runs r
        INNER JOIN arena_runs ar ON r.run_id = ar.run_id
        LEFT JOIN arena_teams at ON r.run_id = at.run_id AND r.username = at.username
        WHERE r.username = $1 AND r.finished = false
        GROUP BY r.run_id, r.username, r.created_at, r.finished, ar.wins, ar.losses
        "#,
        username
    )
    .fetch_optional(pool)
    .await?)
}

async fn get_user_options(
    pool: &Pool<Postgres>,
    username: &String,
) -> Result<Vec<String>, sqlx::Error> {
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
    .await?)
}

pub(crate) async fn show_picks(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<ShowPicksParams>,
) -> Result<Json<Pick>, DataError> {
    dbg!(&params.user);

    let run_info = get_user_current_run(&pool, &params.user).await?;
    let options = get_user_options(&pool, &params.user).await?.iter().map(|s| Pokemon::try_new(s.clone())).collect::<Result<Vec<Pokemon>, DataError>>()?;
    
    let bucket = rand::rng().random_range(0..ARENA_CONFIG.num_buckets);

    let pick = Pick {
        pick_num: 1,
        options: ARENA_POOL
            .get(&bucket)
            .unwrap()
            .choose_multiple(&mut rand::rng(), ARENA_CONFIG.options_per_bucket[bucket])
            .collect::<Vec<&Pokemon>>(),
    };

    Ok(Json(pick))
}
