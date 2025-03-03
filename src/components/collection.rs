use dioxus::prelude::*;

use crate::backend::list_specimens;

#[component]
pub fn Collection() -> Element {
    let specimens = use_server_future(list_specimens)?
        .unwrap()
        .unwrap_or_default();

    rsx! {
        div {
            id: "collection",
            h4 { "My Collection" }
            ul {
                for specimen in specimens {
                    li {
                        key: specimen.id,
                        "{specimen.name} - ({specimen.species:6})"
                    }
                }
            }
        }
    }
}
