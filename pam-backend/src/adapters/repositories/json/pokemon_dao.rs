use serde::Deserialize;

use crate::domain::pokemon::Pokemon;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PokemonDao {
    name: String,
    types: Vec<String>,
    base_species: Option<String>,
    #[serde(default)]
    evos: Vec<String>,
}

impl From<PokemonDao> for Pokemon {
    fn from(pokemon_dao: PokemonDao) -> Self {
        Self {
            id: pokemon_dao.name.clone(),
            name: pokemon_dao.name,
            types: (
                pokemon_dao.types.first().cloned().unwrap_or_default(),
                pokemon_dao.types.get(1).cloned(),
            ),
            base_species: pokemon_dao.base_species,
            evos: pokemon_dao.evos,
        }
    }
}
