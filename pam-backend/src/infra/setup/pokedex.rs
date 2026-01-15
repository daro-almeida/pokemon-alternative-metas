use std::{collections::HashMap, fs};

use once_cell::sync::Lazy;

use crate::domain::pokemon::Pokemon;

const POKEDEX_PATH: &str = "data/pokedex.json";

pub(crate) static POKEDEX: Lazy<HashMap<String, Pokemon>> = Lazy::new(|| {
    load_pokedex().expect("Failed to load pokedex")
});

fn load_pokedex() -> anyhow::Result<HashMap<String, Pokemon>> {
    Ok(serde_json::from_reader(fs::File::open(POKEDEX_PATH)?)?)
}
