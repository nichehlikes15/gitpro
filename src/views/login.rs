use dioxus::{prelude::*};

use crate::routes::Route;
use crate::components::topbar;
use crate::providers::login::tokenhandler;

#[component]
pub fn Login() -> Element {
    use_future(|| async {
        println!("{}", tokenhandler::start_webserver().await);
    });

    rsx! {
        topbar::topbar {}

        div { id: "container",
            img { class: "img", src: asset!("/assets/icons/gitpro.svg"), alt: "branch icon"}
            p { class: "s1", "Welcome to Git Pro" }
            p { class: "s2", "Get started by logging into your github account below:" }
            div { class: "home_buttons",
                a { href: "https://github.com/login/oauth/authorize?client_id=Ov23liICKYW0zOWqK6xS&redirect_uri=http://127.0.0.1:49152/callback&scope=repo", target: "_blank", button { "Link Github" } }
            }
            Link { class: "s3 link", to: Route::Menu {}, "Open Existing Local Repository"}
        }
    }
}