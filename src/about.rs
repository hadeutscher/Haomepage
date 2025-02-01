use yew::prelude::*;

#[function_component(About)]
pub fn footer() -> Html {
    html! {
        <div class={classes!("container", "content")}>
            <p class={classes!("f1")}> { "Hello. I'm Yuval." } </p>
            <p class={classes!("f2")}> { "You might also know me as Deutscher. " } </p>
            <p class={classes!("f3")}> { "I'm a software engineer." } </p>
            <p>
                { "I like video games, operating systems, and programming languages — especially Rust, which I used to create this site.\n" }
                <br />
                { "I enjoy coffee and craft beer, and sometimes dance West Coast Swing.\n" }
                <br />
                { "I'm also interested in philosophy and religion; I've studied a bit of Talmud and am a fan of " } <a href="https://en.wikipedia.org/wiki/Friedrich_Hayek"> { "Friedrich A. Hayek" } </a> { "'s work." }
                <br />
                { "The logo you see here is my own creation, partially crafted by " } <a href="https://github.com/hadeutscher/Haomepage/blob/master/HaLogo.svg?short_path=898e08c"> { "hand-written SVG" } </a> { ", and it's inspired by the Sharingan from the anime series Naruto." }
            </p>
            <p>
                { "You can find my code on " }
                <a href="https://github.com/hadeutscher/"> {"GitHub"} </a>
                {", follow me on " }
                <a href="https://twitter.com/hadeutscher/"> {"Twitter"} </a>
                { ", or connect with me on " }
                <a href="https://www.linkedin.com/in/hadeutscher/"> {"LinkedIn"} </a>
                { "." }
                <br />
                { "You can also reach out to me via email at " }
                <a href="mailto:yuval@deut.sh"> { "yuval@deut.sh" } </a>
                { "."}
            </p>
            <p>
                { "Here are the links to the projects I'm currently working on: " }
                <a href="https://harail.deut.sh/"> { "HaRail" } </a>
                { " (" } <a href="https://github.com/hadeutscher/RustyRail"> { "GitHub" } </a> { ")" }
            </p>
        </div>
    }
}
