use dioxus::prelude::*;

use crate::NotFound;
use crate::about::About;
use crate::contact::Contact;
use crate::home::Home;

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
