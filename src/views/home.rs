use crate::backend::get_collection;
use crate::components::{AddSpecimen, Collection};
use crate::data::UserData;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Home() -> Element {
    let collection = use_server_future(get_collection)?
        .unwrap()
        .unwrap_or_default();

    use_context_provider(|| {
        Signal::new(UserData {
            collection: collection,
            species: HashMap::new(),
        })
    });

    rsx! {
        Collection {}
        AddSpecimen {}
    }
}
