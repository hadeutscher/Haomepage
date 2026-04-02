use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            div { class: "container",
                span { class: "attribution",
                    "© 2026 Yuval Deutscher. "
                    a { href: "https://github.com/hadeutscher/Haomepage",
                        "Source code available on GitHub"
                    }
                    ". "
                }
            }
        }
    }
}
