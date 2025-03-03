use crate::backend::get_collection;
use crate::components::{AddSpecimen, CollectionView};
use crate::data::Collection;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let collection: Collection = use_server_future(get_collection)?.unwrap().unwrap();

    use_context_provider(|| Signal::new(collection));

    rsx! {
        CollectionView {}
        AddSpecimen {}
    }
}
