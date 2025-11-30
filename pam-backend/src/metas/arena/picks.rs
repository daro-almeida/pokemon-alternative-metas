use std::collections::HashMap;

use axum::{Json, extract::Query};
use once_cell::sync::Lazy;
use rand::{self, Rng, seq::IndexedRandom};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::{DataError, Pokemon};

fn load_arena_config() -> Result<ArenaConfig, DataError> {
    let config = read_to_json("src/data/arena/config.json")?;

    let points_to_bucket = config["points_to_bucket"]
        .as_object()
        .ok_or(DataError::Format("points_to_bucket".into()))?
        .iter()
        .map(|(k, v)| {
            let num = k
                .parse::<u32>()
                .map_err(|_| DataError::Format(format!("points_to_bucket: k: {}", k)))?;
            let bucket = v
                .as_u64()
                .ok_or(DataError::Format(format!("points_to_bucket v: {}", v)))?;
            Ok((num, bucket as u32))
        })
        .collect::<Result<HashMap<u32, u32>, DataError>>()?;

    let quotas = config["quotas"]
        .as_array()
        .ok_or(DataError::Format("quotas".into()))?
        .iter()
        .map(|quota| {
            Ok(quota
                .as_u64()
                .ok_or(DataError::Format(format!("{}", quota)))? as u32)
        })
        .collect::<Result<Vec<u32>, DataError>>()?;

    let num_picks = quotas.iter().sum();
    let num_buckets = quotas.len();

    let options_per_bucket = config["options_per_bucket"]
        .as_array()
        .ok_or(DataError::Format("options_per_bucket".into()))?
        .iter()
        .map(|num_options| {
            Ok(num_options
                .as_u64()
                .ok_or(DataError::Format(format!("{}", num_options)))? as u32)
        })
        .collect::<Result<Vec<u32>, DataError>>()?;

    Ok(ArenaConfig {
        points_to_bucket,
        options_per_bucket,
        quotas,
        num_picks,
        num_buckets,
    })
}

fn load_arena_pool(config: &ArenaConfig) -> Result<HashMap<u32, Vec<Pokemon>>, DataError> {
    let mut points_pool = read_to_json("src/data/arena/draft_points.json")?
        .as_object()
        .ok_or(DataError::Format("arena/draft_points.json".into()))?
        .iter()
        .map(|(k, v)| {
            let num = k
                .parse::<u32>()
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
        .collect::<Result<HashMap<u32, Vec<Pokemon>>, DataError>>()?;

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
        |mut acc: HashMap<u32, Vec<Pokemon>>, (points, pokes)| {
            let bucket = *config
                .points_to_bucket
                .get(&points)
                .ok_or(DataError::Format(format!("{} not in config", points)))?;

            acc.entry(bucket).or_default().extend(pokes);
            Ok::<_, DataError>(acc)
        },
    )?;

    Ok(pool)
}

static ARENA_CONFIG: Lazy<ArenaConfig> = Lazy::new(|| load_arena_config().unwrap());

static ARENA_POOL: Lazy<HashMap<u32, Vec<Pokemon>>> =
    Lazy::new(|| load_arena_pool(&ARENA_CONFIG).unwrap());

fn read_to_json(path: &str) -> Result<Value, DataError> {
    let file = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&file)?)
}

struct ArenaConfig {
    points_to_bucket: HashMap<u32, u32>,
    options_per_bucket: Vec<u32>,
    quotas: Vec<u32>,
    num_picks: u32,
    num_buckets: usize,
}

#[derive(Deserialize)]
pub struct ShowPicksParams {
    user: String,
}

#[derive(Serialize)]
pub struct Pick {
    pick_num: u64,
    options: Vec<&'static Pokemon>,
}

pub(crate) async fn show_picks(
    Query(params): Query<ShowPicksParams>,
) -> Result<Json<Pick>, DataError> {
    let user = params.user;
    println!("User is: {}", user);

    // random number between 1 and 19
    //let bucket = rand::rng().random_range(0..=5);

    let pick = Pick {
        pick_num: 1,
        options: ARENA_POOL
            .get(&5)
            .unwrap()
            .choose_multiple(&mut rand::rng(), 10000)
            .collect::<Vec<&Pokemon>>(),
    };

    Ok(Json(pick))
}
