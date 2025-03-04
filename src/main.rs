use crate::backend::get_collection;
use crate::components::{AddSpecimen, CollectionView};
use crate::data::Collection;
use dioxus::prelude::*;

mod backend;
mod components;
mod data;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut collection: Collection = use_server_future(get_collection)?.unwrap().unwrap();
    use_context_provider(|| Signal::new(collection));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        CollectionView {}
        AddSpecimen {}
    }
}
