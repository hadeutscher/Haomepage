use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "container content",
            p { class: "f1", "Hello. I'm Yuval." }
            p { class: "f2", "You might also know me as Deutscher. " }
            p { class: "f3", "I'm a software engineer." }
            p {
                "I like video games, operating systems, and programming languages — especially Rust, which I used to create this site."
                br {}
                "I enjoy coffee and craft beer, and sometimes dance West Coast Swing."
                br {}
                "I'm also interested in philosophy and religion; I've studied a bit of Talmud and am a fan of "
                a { href: "https://en.wikipedia.org/wiki/Friedrich_Hayek", "Friedrich A. Hayek" }
                "'s work."
                br {}
                "The logo you see here is my own creation, partially crafted by "
                a { href: "https://github.com/hadeutscher/Haomepage/blob/master/HaLogo.svg?short_path=898e08c",
                    "hand-written SVG"
                }
                ", and it's inspired by the Sharingan from the anime series Naruto."
            }
            p {
                "You can find my code on "
                a { href: "https://github.com/hadeutscher/", "GitHub" }
                ", follow me on "
                a { href: "https://twitter.com/hadeutscher/", "Twitter" }
                ", or connect with me on "
                a { href: "https://www.linkedin.com/in/hadeutscher/", "LinkedIn" }
                "."
                br {}
                "You can also reach out to me via email at "
                a { href: "mailto:yuval@deut.sh", "yuval@deut.sh" }
                "."
            }
            p {
                "Here are the links to the projects I'm currently working on: "
                a { href: "https://harail.deut.sh/", "HaRail" }
                " ("
                a { href: "https://github.com/hadeutscher/RustyRail", "GitHub" }
                ")"
            }
        }
    }
}
