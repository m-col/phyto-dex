use crate::backend::{get_collection, list_species};
use crate::components::{AddSpecimen, Collection};
use crate::data::UserData;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Home() -> Element {
    let collection = use_server_future(get_collection)?
        .unwrap()
        .unwrap_or_default();

    let species = use_server_future(list_species)?
        .unwrap()
        .unwrap_or_default();

    use_context_provider(|| {
        Signal::new(UserData {
            collection: collection,
            species: species.into_iter().map(|s| (s.id, s)).collect(),
        })
    });

    rsx! {
        Collection {}
        AddSpecimen {}
    }
}
