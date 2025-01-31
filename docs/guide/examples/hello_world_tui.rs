#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_tui::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            width: "100%",
            height: "10px",
            background_color: "red",
            justify_content: "center",
            align_items: "center",

            "Hello world!"
        }
    })
}
