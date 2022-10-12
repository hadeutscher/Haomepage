use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

fn nav_link(text: &str, route: AppRoute) -> Html {
    html! {
        <li class={classes!("nav-item")}>
                    <Link<AppRoute> to={route} classes={classes!("nav-link")}>
                        { text }
                    </Link<AppRoute>>
        </li>
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class={classes!("navbar", "navbar-dark")}>
            <div class={classes!("container")}>
                <Link<AppRoute> to={AppRoute::Home} classes={classes!("navbar-brand")}>
                    { "HaDeutscher" }
                </Link<AppRoute>>
                <ul class={classes!("nav", "navbar-nav", "pull-xs-right")}>
                    { nav_link("Home", AppRoute::Home) }
                    { nav_link("About", AppRoute::About) }
                </ul>
            </div>
        </nav>
    }
}
