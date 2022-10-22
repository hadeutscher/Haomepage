use yew::prelude::*;

#[function_component(About)]
pub fn footer() -> Html {
    html! { 
        <div class={classes!("container", "content")}>
            <p class={classes!("f1")}> { "Hey there." } </p>
            <p class={classes!("f2")}> { "I'm Yuval, but most people call me Deutscher. " } </p>
            <p>
                { "I'm a software engineer." } <br /> { "I like video games, operating systems, and programming languages (especially Rust, which this site is written in).\n" }
                <br />
                { "I like freshly roasted coffee and craft beer. I also sometimes dance West Coast Swing.\n" }
                <br />
                { "I'm also interested in philosophy and religion, and study a little bit of Talmud."}
                <br />
                { "I drew my logo on my own, partially by " }
                <a href="https://github.com/hadeutscher/Haomepage/blob/master/HaLogo.svg?short_path=898e08c"> { "hand-written SVG" } </a>
                { ". It's based on the Sharingan from the anime series Naruto." }
            </p>
            <p>
                { "You can find my code on " }
                <a href="https://github.com/hadeutscher/"> {"GitHub"} </a>
                {", follow me on " }
                <a href="https://twitter.com/hadeutscher/"> {"Twitter"} </a>
                { " or connect with me on " }
                <a href="https://www.linkedin.com/in/hadeutscher/"> {"LinkedIn"} </a>
                { "." }
                <br />
                { "You can also send me an e-mail to " }
                <a href="mailto:yuval@deut.sh"> {"yuval@deut.sh"} </a>
                { "."}
            </p>
        </div>
    }
}
