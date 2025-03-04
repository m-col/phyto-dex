use dioxus::logger::tracing::warn;
use dioxus::prelude::*;

use crate::backend::{add_specimen, list_species, get_species_by_id};
use crate::data::*;

#[component]
pub fn AddSpecimen() -> Element {
    let mut form_name = use_signal(|| String::new());
    let mut form_species = use_signal(|| 0);

    let available_species = use_server_future(list_species)?()
        .unwrap()
        .unwrap_or_default();

    let mut collection: Signal<Collection> = use_context();

    rsx! {
        div { id: "add-specimen",
            h4 { "Add a new specimen" }
            input {
                placeholder: "Enter name...",
                value: "{form_name}",
                oninput: move |event| form_name.set(event.value()),
            }
            select { onchange: move |event| form_species.set(event.value().parse::<i32>().unwrap()),
                option { value: 0, "Select a species..." }
                for species in available_species {
                    option { key: species.id, value: species.id, "{species.name}" }
                }
            }
            button {
                onclick: move |_| async move {
                    warn!("Add specimen");
                    let name = form_name();
                    let species_id = form_species();
                    match add_specimen(name.clone(), species_id).await {
                        Ok(specimen_id) => {
                            let mut collection = &mut collection.write();
                            collection
                                .specimens
                                .push(Specimen {
                                    id: specimen_id,
                                    name: name,
                                    species_id: species_id,
                                });
                            if !collection.species.contains_key(&species_id) {
                                let species = get_species_by_id(species_id).await.unwrap();
                                collection.species.insert(species_id, species);
                            }
                        }
                        Err(err) => warn!("Error: {}", err),
                    }
                },
                "Add"
            }
        }
    }
}
