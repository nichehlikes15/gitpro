use dioxus::{prelude::*};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Style {
            r#type: "text/css",
            {include_str!("../assets/main.css")}
        }

        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Menu {}
    }
}

#[component]
pub fn Menu() -> Element {
    rsx! {
        div { 
            id: "buttons",
            button { class: "button", "Push" }
            button { class: "button", "Pull" }
            button { class: "button", "Commit" }
        }
    }
}
