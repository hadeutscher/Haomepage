use dioxus::prelude::*;

use crate::NotFound;
use crate::about::About;
use crate::home::Home;

#[derive(Routable, Clone, PartialEq)]
pub enum AppRoute {
    #[layout(crate::Layout)]
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
