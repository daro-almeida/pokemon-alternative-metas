use crate::{
    adapters::repositories::json::JsonPokemonRepository,
    application::{AppError, AppResult, repositories::pokemon::PokemonRepository},
    domain::pokemon::Pokemon,
};

impl PokemonRepository for JsonPokemonRepository {
    fn get_by_id(&self, id: &str) -> AppResult<&'static Pokemon> {
        self.pokedex
            .get(id)
            .ok_or_else(|| AppError::Database(format!("Pokemon with id {} not found", id)))
    }
}
