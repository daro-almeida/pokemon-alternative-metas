use std::{collections::HashMap, fs};

use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::Value;

use crate::domain::pokemon::Pokemon;

const POKEDEX_PATH: &str = "data/pokedex.json";

pub(crate) static POKEDEX: Lazy<HashMap<String, Pokemon>> =
    Lazy::new(|| load_pokedex().expect("Failed to load pokedex"));

fn load_pokedex() -> anyhow::Result<HashMap<String, Pokemon>> {
    #[derive(Deserialize)]
struct PokemonData {
    name: String,
    types: Vec<String>,
    #[serde(default)]
    evos: Vec<String>,
}

let raw: HashMap<String, Value> = serde_json::from_reader(fs::File::open(POKEDEX_PATH)?)?;

raw
    .into_iter() 
    .filter_map(|(id, data)| {
        if data.get("isCosmeticForme").and_then(|v| v.as_bool()).unwrap_or(false) {
            return None;
        }

        // Try deserializing as PokemonData
        let poke_data = match serde_json::from_value::<PokemonData>(data) {
            Ok(poke_data) => poke_data,
            Err(e) => return Some(Err(anyhow::anyhow!(e))),
        };

        let pokemon = Pokemon {
            id: id.clone(), 
            name: poke_data.name,
            types: (
                poke_data.types.get(0).cloned().unwrap_or_default(),
                poke_data.types.get(1).cloned(),
            ),
            evos: poke_data.evos,
        };
        
        Some(Ok((id, pokemon))) 
    })
    .collect::<anyhow::Result<HashMap<String, Pokemon>>>()
}
