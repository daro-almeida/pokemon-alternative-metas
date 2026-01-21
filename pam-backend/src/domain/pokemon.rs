use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pokemon {
    pub id: String,
    pub name: String,
    pub types: (String, Option<String>),
    #[serde(skip_serializing)]
    pub base_species: Option<String>,
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

    pub fn same_base_species(&self, other: &Self) -> bool {
        match (self.base_species.as_deref(), other.base_species.as_deref()) {
            (Some(bs1), Some(bs2)) => bs1 == bs2,
            (Some(bs), None) => bs == other.name,
            (None, Some(bs)) => bs == self.name,
            (None, None) => false,
        }
    }
}

impl PartialEq for Pokemon {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
