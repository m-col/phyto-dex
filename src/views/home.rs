use crate::components::{AddSpecimen, Collection};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Collection {}
        AddSpecimen {}
    }
}
