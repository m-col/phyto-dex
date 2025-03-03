use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type SpeciesId = i32;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Species {
    id: SpeciesId,
    name: String,
    genus: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Specimen {
    pub id: i32,
    pub name: String,
    pub species: SpeciesId,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserData {
    pub collection: Vec<Specimen>,
    pub species: HashMap<SpeciesId, Species>,
}
