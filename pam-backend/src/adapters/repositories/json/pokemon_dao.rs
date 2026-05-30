use serde::Deserialize;

use crate::domain::pokemon::Pokemon;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PokemonDao {
    pub(crate) name: String,
    pub(crate) types: Vec<String>,
    pub(crate) base_species: Option<String>,
    #[serde(default)]
    pub(crate) evos: Vec<String>,
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
