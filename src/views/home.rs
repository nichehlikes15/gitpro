use dioxus::{prelude::*};

use crate::providers;
use crate::routes::Route;
use crate::components::topbar;

#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();
    let token_check = use_resource(|| async move {
        providers::login::checklogin::check_token()
            .await
            .unwrap_or(false)
    });
    dbg!(token_check());

    use_effect(move || {
        if matches!(token_check(), Some(false)) {
            navigator.push(Route::Login {});
        } else if matches!(token_check(), Some(true)) {
        	navigator.push(Route::Menu {});
        }
    });

    rsx! {
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
