use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div { class: "container content",
            p { "I do consulting work on everything software, from systems programming to DevOps." }
            p {
                "For personal or professional inquiries, the best way to reach me is via email at "
                a { href: "mailto:yuval@deut.sh", "yuval@deut.sh" }
                ". I read all non-spam emails."
            }
        }
    }
}
