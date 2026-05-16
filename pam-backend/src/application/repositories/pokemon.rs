use crate::{application::AppResult, domain::pokemon::Pokemon};

pub trait PokemonRepository: Send + Sync {
    fn get_by_id(&self, id: &str) -> AppResult<&'static Pokemon>;
}
