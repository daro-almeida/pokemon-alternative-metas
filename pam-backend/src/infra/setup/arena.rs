use std::{collections::HashMap, fs, sync::Arc};

use anyhow::Context;
use serde_json::Value;

use crate::{
    application::use_cases::arena::{Arena, ArenaConfig, ArenaPersistence},
    domain::pokemon::Pokemon,
};

const CONFIG_PATH: &str = "data/arena/config.json";
const DRAFT_POINTS_PATH: &str = "data/arena/draft_points.json";
const EXCEPTIONS_PATH: &str = "data/arena/exceptions.json";

fn load_arena_config() -> anyhow::Result<ArenaConfig> {
    Ok(serde_json::from_reader(fs::File::open(CONFIG_PATH)?)?)
}

fn load_arena_pool(
    config: &ArenaConfig,
    pokedex: &'static HashMap<String, Pokemon>,
) -> anyhow::Result<HashMap<usize, Vec<&'static Pokemon>>> {
    let draft_points_json: Value = serde_json::from_reader(fs::File::open(DRAFT_POINTS_PATH)?)?;
    let mut draft_points_pool = draft_points_json
        .as_object()
        .context("arena/draft_points.json is not an object")?
        .iter()
        .map(|(points, poke_ids)| {
            let points: usize = points.parse::<usize>()?;

            let pokemons_list = poke_ids
                .as_array()
                .context("arena/draft_points.json: value is not an array")?
                .iter()
                .map(|poke_id| {
                    Ok(pokedex
                        .get(
                            poke_id
                                .as_str()
                                .context("arena/draft_points.json: arrays are not of strings")?,
                        )
                        .context("arena/draft_points.json: pokemon not found in pokedex")?)
                })
                .collect::<anyhow::Result<Vec<&Pokemon>>>()?;

            Ok((points, pokemons_list))
        })
        .collect::<anyhow::Result<HashMap<usize, Vec<&Pokemon>>>>()?;

    let exceptions_json: Value = serde_json::from_reader(fs::File::open(EXCEPTIONS_PATH)?)?;
    let exceptions: Vec<String> = exceptions_json
        .as_array()
        .context("arena/exceptions.json is not an array")?
        .iter()
        .map(|pid| {
            Ok(pid
                .as_str().context("arena/exceptions.json: array is not of strings")?
                .into())
        })
        .collect::<anyhow::Result<Vec<String>>>()?;

    draft_points_pool
        .get_mut(&1).context("arena/draft_points.json: point 1 list missing")?
        .retain(|p| !p.has_evo() || exceptions.contains(&p.id));

    let pool = draft_points_pool.into_iter().try_fold(
        HashMap::new(),
        |mut acc: HashMap<usize, Vec<&Pokemon>>, (points, pokes)| -> anyhow::Result<HashMap<usize, Vec<&Pokemon>>> {
            let bucket = config
                .points_to_bucket
                .get(&points).context(format!("arena/draft_points.json: point {} not in config", points))?;

            acc.entry(*bucket).or_default().extend(pokes);
            Ok(acc)
        },
    )?;

    Ok(pool)
}

pub fn load_arena(
    pokedex: &'static HashMap<String, Pokemon>,
    persistence: Arc<dyn ArenaPersistence>,
) -> anyhow::Result<Arena> {
    let config = load_arena_config()?;
    let pool = load_arena_pool(&config, pokedex)?;

    Ok(Arena::new(pool, persistence, config))
}
