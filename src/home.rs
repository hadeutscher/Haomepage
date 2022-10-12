use yew::prelude::*;

#[function_component(Home)]
pub fn footer() -> Html {
    html! {
        <div class={classes!("container", "prominent")}>
            <img class={classes!("halogo")} src="HaLogo.svg" />
            <p>{ "I am leet robot" }</p>
        </div>
    }
}
