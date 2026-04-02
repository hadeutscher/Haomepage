use dioxus::prelude::*;

use crate::HALOGO;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "container prominent",
            img { class: "halogo", src: HALOGO }
            p { "I am leet robot" }
        }
    }
}
