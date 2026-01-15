use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pokemon {
    pub id: String,
    pub num: u64,
    pub name: String,
    pub types: (String, Option<String>),
    #[serde(skip_serializing)]
    pub evos: Vec<String>,
}

impl Pokemon {
    pub fn has_evo(&self) -> bool {
        !self.evos.is_empty()
    }

    pub fn is_mega(&self) -> bool {
        self.name.contains("-Mega")
    }
}