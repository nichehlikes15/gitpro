use dioxus::{prelude::*};

#[component]
pub fn push_menu() -> Element {
    let mut repo_link = use_signal(|| "".to_string());
    let mut commit_message = use_signal(|| "".to_string());

    rsx! {
        div { id: "topbar",
            /*p {
                "Current Branch: {current_branch()} Current Repo: {current_repo()}"
            }*/
            
            div { class: "active-branch",
                span { class: "label", "Active branch" }
                div { class: "branch-selection",
                    img {src: asset!("/assets/icons/fork_right.svg"), alt: "branch icon"}
                    "current branch"
                    //"{current_branch()}"
                    img {src: asset!("/assets/icons/arrow_down.svg"), alt: "down icon"}
                }
            }

        },

        div { id: "buttons",
            input {
                class: "input",
                placeholder: "Enter repo link",
                value: "{repo_link()}",
                oninput: move |event| repo_link.set(event.value()),
            },
            input {
                class: "input",
                placeholder: "Enter commit message",
                value: "{commit_message()}",
                oninput: move |event| commit_message.set(event.value()),
            },
            button {
                class: "button",
                "Set Repo"
            },
            button {
                class: "button",
                "Push"
            },
            button { class: "button", "Pull" }
            button { class: "button", "Commit" }
        }
    }
}