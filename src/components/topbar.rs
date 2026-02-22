use dioxus::{prelude::*};

use crate::routes::Route;
use crate::providers::git_helper;
use crate::providers::login;

#[component]
pub fn topbar() -> Element {
    rsx! {
        div { id: "topbar",
            //PushMenu {}
            /*p {
                "Current Branch: {current_branch()} Current Repo: {current_repo()}"
            }*/

            Link { to: Route::Home {}, 
                img { class: "topbar_img", src: asset!("/assets/icons/gitpro.svg"), alt: "branch icon"}
            }
            
            div { class: "active-branch",
                span { class: "label", "Active branch:" }
                div { class: "branch-selection",
                    img {src: asset!("/assets/icons/fork_right.svg"), alt: "branch icon"}
                    "{git_helper::current_branch()}"
                    img {src: asset!("/assets/icons/arrow_down.svg"), alt: "down icon"}
                }
                span { class: "label", "{login::github_username_or_not_logged_in()}" }
            }

        },
    }
}