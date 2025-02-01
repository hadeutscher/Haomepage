use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class={classes!("container")}>
                <span class={classes!("attribution")}>
                    { "© 2024 Yuval Deutscher. Source code available on " }
                    <a href="https://github.com/hadeutscher/Haomepage"> { "GitHub" } </a> { ". " }
                </span>
                <span class={classes!("footer-icons")}>
                    <a href="https://ready.chair6.net/?url=deut.sh">
                        <img src="World_IPv6_launch_logo.svg" height=24 alt="IPv6 Ready" title="IPv6 Ready" />
                    </a>
                </span>
            </div>
        </footer>
    }
}
