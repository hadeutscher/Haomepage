mod about;
mod footer;
mod header;
mod home;
mod routes;

use yew::prelude::*;
use yew_router::prelude::*;

use about::About;
use footer::Footer;
use header::Header;
use home::Home;
use routes::AppRoute;

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::NotFound => {
            html! { <p class={classes!("container", "prominent")}>{ r"¯\_(ツ)_/¯" }</p> }
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <Switch<AppRoute> render={switch} />
            <Footer />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
