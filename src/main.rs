#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::*, html::{h1, script}};

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    render!(rsx! {
        div {
            h1 { "BUDDYY YOUR CPU IS BOUTTA ☠️EXPLODE☠️" }
            script {include_str!("index.js")}
        }
    })
}
