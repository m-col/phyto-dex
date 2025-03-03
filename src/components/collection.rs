use dioxus::prelude::*;

use crate::data::Collection;

#[component]
pub fn CollectionView() -> Element {
    let read_collection: Signal<Collection> = use_context();

    rsx! {
        div { id: "collection",
            h4 { "My Collection" }
            ul {
                {
                    let collection = read_collection();
                    rsx! {
                        for specimen in collection.specimens {
                            {
                                let species = collection.species.get(&specimen.species_id).unwrap();
                                rsx! {
                                    li { key: specimen.id, "{specimen.name} - ({species.name})" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
