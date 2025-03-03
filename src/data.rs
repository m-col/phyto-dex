use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type SpeciesId = i32;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Species {
    pub id: SpeciesId,
    pub name: String,
    pub genus: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Specimen {
    pub id: i32,
    pub name: String,
    pub species_id: SpeciesId,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserData {
    pub collection: Vec<Specimen>,
    pub species: HashMap<SpeciesId, Species>,
}
