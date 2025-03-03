use dioxus::prelude::*;

use crate::backend::get_collection;
use crate::data::UserData;

#[component]
pub fn Collection() -> Element {
    let read_user_data: Signal<UserData> = use_context();

    rsx! {
        div { id: "collection",
            h4 { "My Collection" }
            ul {
                {
                    let user_data = read_user_data();
                    rsx! {
                        for specimen in user_data.collection {
                            {
                                let species = match user_data.species.get(&specimen.species_id) {
                                    Some(entry) => &entry.name,
                                    None => &"Unknown".to_string(),
                                };
                                rsx! {
                                    li { key: specimen.id, "{specimen.name} - ({species})" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
