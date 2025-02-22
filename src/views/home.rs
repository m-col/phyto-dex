use crate::components::{AddSpecimen, Hero};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        AddSpecimen {}
    }
}
