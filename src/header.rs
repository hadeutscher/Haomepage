use dioxus::prelude::*;

use crate::routes::AppRoute;

#[component]
fn NavLink(text: String, route: AppRoute) -> Element {
    rsx! {
        li { class: "nav-item",
            Link { to: route, class: "nav-link", "{text}" }
        }
    }
}

#[component]
pub fn Header() -> Element {
    rsx! {
        nav { class: "navbar navbar-dark",
            div { class: "container",
                Link { to: AppRoute::Home {}, class: "navbar-brand", "HaDeutscher" }
                ul { class: "nav navbar-nav pull-xs-right",
                    NavLink { text: "Home", route: AppRoute::Home {} }
                    NavLink { text: "About", route: AppRoute::About {} }
                    NavLink { text: "Contact", route: AppRoute::Contact {} }
                }
            }
        }
    }
}
