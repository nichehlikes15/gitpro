use dioxus::{prelude::*};

use crate::components::topbar;
use crate::views::menu;

#[component]
pub fn home() -> Element {
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
            p { class: "s3 link",
                onclick: move |_| {menu::menu {};}, 
                "Open Existing Local Repository",
            }
        }
    }
}