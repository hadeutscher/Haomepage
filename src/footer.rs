use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            div { class: "container",
                span { class: "attribution",
                    "© 2024 Yuval Deutscher. Source code available on "
                    a { href: "https://github.com/hadeutscher/Haomepage", "GitHub" }
                    ". "
                }
                span { class: "footer-icons",
                    a { href: "https://ready.chair6.net/?url=deut.sh",
                        img {
                            src: "World_IPv6_launch_logo.svg",
                            height: "24",
                            alt: "IPv6 Ready",
                            title: "IPv6 Ready",
                        }
                    }
                }
            }
        }
    }
}
