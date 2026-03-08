use dioxus::{prelude::*};

use crate::routes::Route;
use crate::providers::git_helper_old;
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
                img { class: "topbar_img", src: asset!("/assets/icons/gitpro.svg"), alt: "branch"}
            }

            img { class: "topbar_img", src: asset!("/assets/icons/star.svg"), alt: "star"}
            
            div { class: "active-branch",
                span { class: "label", "Active branch:" }
                div { class: "branch-selection",
                    img {src: asset!("/assets/icons/fork_right.svg"), alt: "branch"}
                    "test" //"{git_helper::current_branch()}"
                    img {src: asset!("/assets/icons/arrow_down.svg"), alt: "down"}
                }
                //span { class: "label", "{login::github_username_or_not_logged_in()}" }
            }

        },
    }
}