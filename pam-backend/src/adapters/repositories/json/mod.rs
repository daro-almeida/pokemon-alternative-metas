use std::{collections::HashMap, fs};

use serde_json::Value;
use std::sync::LazyLock;

use crate::startup::paths::POKEDEX_PATH;
use crate::{adapters::repositories::json::pokemon_dao::PokemonDao, domain::pokemon::Pokemon};

pub mod pokemon_dao;
pub mod pokemon_repository;

static POKEDEX: LazyLock<HashMap<String, Pokemon>> =
    LazyLock::new(|| load_pokedex().expect("Failed to load pokedex"));

fn load_pokedex() -> anyhow::Result<HashMap<String, Pokemon>> {
    let raw: HashMap<String, Value> = serde_json::from_reader(fs::File::open(POKEDEX_PATH)?)?;

    raw.into_iter()
        .filter_map(|(id, data)| {
            if data
                .get("isCosmeticForme")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                return None;
            }

            let pokemon_dao = match serde_json::from_value::<PokemonDao>(data) {
                Ok(pokemon_dao) => pokemon_dao,
                Err(e) => return Some(Err(anyhow::anyhow!(e))),
            };

            let pokemon = Pokemon {
                id: id.clone(),
                name: pokemon_dao.name,
                types: (
                    pokemon_dao.types.get(0).cloned().unwrap_or_default(),
                    pokemon_dao.types.get(1).cloned(),
                ),
                base_species: pokemon_dao.base_species,
                evos: pokemon_dao.evos,
            };

            Some(Ok((id, pokemon)))
        })
        .collect::<anyhow::Result<HashMap<String, Pokemon>>>()
}

pub struct JsonPokemonRepository {
    pokedex: &'static HashMap<String, Pokemon>,
}

impl Default for JsonPokemonRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonPokemonRepository {
    pub fn new() -> Self {
        Self { pokedex: &POKEDEX }
    }
}
