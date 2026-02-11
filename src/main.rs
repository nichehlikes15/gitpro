use dioxus::{prelude::*};

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
        Menu {}
    }
}

#[component]
pub fn Menu() -> Element {
    rsx! {
        div { id: "buttons",
            button { class: "button", "Push" }
            button { class: "button", "Pull" }
            button { class: "button", "Commit" }
        }
    }
}
