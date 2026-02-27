//Run with dx serve --always-on-top false
//https://github.com/login/oauth/authorize?client_id=Ov23liICKYW0zOWqK6xS&redirect_uri=http://127.0.0.1:49152/callback&scope=repo

use dioxus::prelude::*;

mod components;
mod views;
mod providers;
mod routes;

use routes::Route;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const JETBRAINS_MONO: Asset = asset!("/assets/fonts/JetBrainsMono-Medium.ttf");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Style { r#type: "text/css",
            {
                format!(
                    "@font-face {{ font-family: 'JetBrainsMono'; src: url('{}') format('truetype'); font-weight: normal; font-style: normal; }}",
                    JETBRAINS_MONO,
                )
            }
        }

        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
        //views::menu::menu {}
        //views::home::home {}
    }
}