use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type SpeciesId = i32;
pub type GenusId = i32;
pub type SpecimenId = i32;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Species {
    pub id: SpeciesId,
    pub name: String,
    pub genus: GenusId,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Specimen {
    pub id: SpecimenId,
    pub name: String,
    pub species_id: SpeciesId,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Collection {
    pub specimens: Vec<Specimen>,
    pub species: HashMap<SpeciesId, Species>,
}
