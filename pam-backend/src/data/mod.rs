use axum::{http::StatusCode, response::IntoResponse};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

const POKEDEX_PATH: &str = "src/data/pokedex.json";

static POKEDEX: Lazy<Value> = Lazy::new(|| {
    let file =
        std::fs::read_to_string(POKEDEX_PATH).expect("Failed to read pokedex.json");

    serde_json::from_str(&file).expect("Failed to parse pokedex.json")
});

#[derive(Error, Debug)]
pub enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Structural or semantic issues with the JSON content
    #[error("Malformed data: {0}")]
    Format(String),

    #[error("Value not found: {0}")]
    NotFound(String),

    #[error("SQL error: {0}")]
    Sql(#[from] sqlx::Error),
}

#[derive(Serialize, Debug, Clone)]
pub struct Pokemon {
    pub(crate) id: String,
    num: u64,
    name: String,
    types: (String, Option<String>),
    #[serde(skip_serializing)]
    evos: Vec<String>,
}


impl IntoResponse for DataError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error fetching Pokémon: {}", self),
        )
            .into_response()
    }
}

impl Pokemon {
    pub fn try_new(id: String) -> Result<Self, DataError> {
        let poke_object = POKEDEX.get(&id).ok_or(DataError::NotFound(id.clone()))?;

        let num = poke_object["num"]
            .as_u64()
            .ok_or(DataError::Format(format!("{}, {}", id.clone(), "num")))?;

        let name = poke_object["name"]
            .as_str()
            .ok_or(DataError::Format(format!("{}, {}", id.clone(), "name")))?
            .to_string();

        let types_vec = poke_object["types"]
            .as_array()
            .ok_or(DataError::Format(format!("{}, {}", id.clone(), "types")))?;

        let types = match types_vec.as_slice() {
            [t1] => (
                t1.as_str()
                    .ok_or(DataError::Format(format!("{}, {}", id.clone(), "types")))?
                    .to_string(),
                None,
            ),
            [t1, t2] => (
                t1.as_str()
                    .ok_or(DataError::Format(format!("{}, {}", id.clone(), "types")))?
                    .to_string(),
                Some(
                    t2.as_str()
                        .ok_or(DataError::Format(format!("{}, {}", id.clone(), "types")))?
                        .to_string(),
                ),
            ),
            _ => return Err(DataError::Format(format!("{}, {}", id.clone(), "types"))),
        };

        let evos = poke_object["evos"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|evo| {
                Ok(evo
                    .as_str()
                    .ok_or(DataError::Format(format!("{}, {}", id.clone(), "evos")))?
                    .to_string())
            })
            .collect::<Result<Vec<String>, DataError>>()?;

        Ok(Pokemon {
            id,
            num,
            name,
            types,
            evos,
        })
    }

    pub fn has_evo(&self) -> bool {
        !self.evos.is_empty()
    }

    pub fn is_mega(&self) -> bool {
        self.name.contains("-Mega")
    }
}
