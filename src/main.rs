mod about;
mod contact;
mod footer;
mod header;
mod home;
mod routes;

use dioxus::prelude::*;

use footer::Footer;
use header::Header;
use routes::AppRoute;

const STYLE: Asset = asset!("/assets/style.scss");
const TACHYONS: Asset = asset!("/assets/tachyons.min.css");
const HALOGO: Asset = asset!("/assets/HaLogo.svg");

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "main-container",
            Header {}
            Outlet::<AppRoute> {}
            Footer {}
        }
    }
}

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        p { class: "container prominent", r"¯\_(ツ)_/¯" }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: HALOGO }
        document::Stylesheet { href: STYLE }
        document::Stylesheet { href: TACHYONS }
        Router::<AppRoute> {}
    }
}

fn main() {
    dioxus::launch(App);
}
