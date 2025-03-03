use dioxus::prelude::*;

use crate::backend::get_collection;
use crate::data::UserData;

#[component]
pub fn Collection() -> Element {
    let user_data: Signal<UserData> = use_context();

    rsx! {
        div { id: "collection",
            h4 { "My Collection" }
            ul {
                for specimen in user_data().collection {
                    li { key: specimen.id, "{specimen.name} - ({specimen.species:6})" }
                }
            }
        }
    }
}
