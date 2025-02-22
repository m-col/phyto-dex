use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            a { href: "https://dioxuslabs.com/learn/0.6/", "Dioxus Docs" }
        }
    }
}
