mod footer;
mod header;
mod home;

use dioxus::prelude::*;

use dioxus_markdown::Markdown;
use footer::Footer;
use header::Header;
use home::Home;

const STYLE: Asset = asset!("/assets/style.scss");
const TACHYONS: Asset = asset!("/assets/tachyons.min.css");
const HALOGO: Asset = asset!("/assets/HaLogo.svg");

#[derive(Routable, Clone, PartialEq)]
pub enum AppRoute {
    #[layout(crate::Layout)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/contact")]
    Contact {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[component]
pub fn MarkdownPage(content: &'static str) -> Element {
    rsx! {
        div { class: "container content",
            Markdown { src: content }
        }
    }
}

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
pub fn About() -> Element {
    rsx! {
        MarkdownPage { content: include_str!("../assets/about.md") }
    }
}

#[component]
pub fn Contact() -> Element {
    rsx! {
        MarkdownPage { content: include_str!("../assets/contact.md") }
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
