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
fn app() -> Element {
    rsx! {
        document::Link { rel: "icon", href: HALOGO }
        document::Stylesheet { href: STYLE }
        document::Stylesheet { href: TACHYONS }
        Router::<AppRoute> {}
    }
}

// The server function at the endpoint "static_routes" will be called by the CLI to generate the list of static
// routes. You must explicitly set the endpoint to `"static_routes"` in the server function attribute instead of
// the default randomly generated endpoint.
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(AppRoute::static_routes()
        .iter()
        .map(ToString::to_string)
        .chain(std::iter::once("/404".to_string()))
        .collect())
}

fn main() {
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(app);
}
