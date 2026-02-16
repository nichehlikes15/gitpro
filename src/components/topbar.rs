use dioxus::{prelude::*};

use crate::providers::git_helper;

#[component]
pub fn topbar() -> Element {
    rsx! {
        div { id: "topbar",
            //PushMenu {}
            /*p {
                "Current Branch: {current_branch()} Current Repo: {current_repo()}"
            }*/
            
            div { class: "active-branch",
                span { class: "label", "Active branch" }
                div { class: "branch-selection",
                    img {src: asset!("/assets/icons/fork_right.svg"), alt: "branch icon"}
                    "{git_helper::current_branch()}"
                    img {src: asset!("/assets/icons/arrow_down.svg"), alt: "down icon"}
                }
            }

        },
    }
}