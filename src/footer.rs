use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class={classes!("container")}>
                <span class={classes!("attribution")}>
                    { "© 2022 Yuval Deutscher. Source code available on " }
                    <a href="https://github.com/hadeutscher/Haomepage"> { "GitHub" } </a>
                </span>
            </div>
        </footer>
    }
}
