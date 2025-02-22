use dioxus::logger::tracing::warn;
use dioxus::prelude::*;
use std::io::Write;

use crate::backend::list_specimens;

#[component]
pub fn Collection() -> Element {
    let specimens = use_server_future(list_specimens)?()
        .unwrap()
        .unwrap_or_default();

    rsx! {
        div {
            id: "collection",
            h4 { "My Collection" }
            ul {
                for (key, name, species) in specimens {
                    li {
                        key: key,
                        "{name} - ({species})"
                    }
                }
            }
        }
    }
}
