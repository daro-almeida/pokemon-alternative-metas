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
        self.base_species == other.base_species
            || self
                .base_species
                .as_deref()
                .is_some_and(|bs| bs == other.name)
            || other
                .base_species
                .as_deref()
                .is_some_and(|bs| bs == self.name)
    }
}

impl PartialEq for Pokemon {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
