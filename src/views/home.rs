use dioxus::{prelude::*};

use crate::providers;
use crate::routes::Route;
use crate::components::topbar;

#[component]
pub fn Home() -> Element {

    let mut token_check = use_signal::<bool>(|| false);
    spawn(async move {
        token_check.set(providers::login::checklogin::check_token().await.unwrap_or(false));
    });

    rsx! {
        if token_check.read().clone() {
            Route::Login {}
        }
        
        topbar::topbar {}

        div { id: "container",
            img { class: "img", src: asset!("/assets/icons/gitpro.svg"), alt: "branch icon"}
            p { class: "s1", "Welcome to Git Pro" }
            p { class: "s2", "Get started by cloning or creating a Repository" }
            div { class: "home_buttons",
                button { "Clone a Repository" }
                button { "Create new Repository" }
            }
            Link { class: "s3 link", to: Route::Menu {}, "Open Existing Local Repository"}
        }
    }
}