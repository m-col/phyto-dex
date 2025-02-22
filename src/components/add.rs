use dioxus::logger::tracing::warn;
use dioxus::prelude::*;
use std::io::Write;

use crate::backend::{add_specimen, list_species};

#[component]
pub fn AddSpecimen() -> Element {
    let mut form_name = use_signal(|| String::new());
    let mut form_species = use_signal(|| 0);

    //let available_species = use_resource(list_species).suspend()?;

    rsx! {
        div {
            id: "add-specimen",
            h4 { "Add a new specimen" }
            input {
                placeholder: "Enter name...",
                value: "{form_name}",
                oninput: move |event| form_name.set(event.value())
            }
            //select {
            //    onchange: move |event| form_species.set(event.value().parse::<i32>().unwrap()),
            //    option {
            //        value: 0,
            //        "Select a species..."
            //    }
            //    for (key, name) in available_species().unwrap() {
            //        option {
            //            key: key,
            //            value: key,
            //            "{name}"
            //        }
            //    }
            //}
            button {
                onclick: move |_| async move {
                    warn!("Add specimen");
                    add_specimen(form_name(), form_species()).await;
                },
                "Add"
            }

        }
    }
}
