use dioxus::{prelude::*};

use url::Url;

use crate::components::topbar;
use crate::providers::git_helper;

#[component]
pub fn menu() -> Element {
    let mut repo_link = use_signal(|| "".to_string());
    let mut commit_message = use_signal(|| "".to_string());

    rsx! {
        topbar::topbar {}

        div { id: "buttons",
            input {
                class: "input",
                placeholder: git_helper::current_repo(),
                value: repo_link(),
                oninput: move |event| repo_link.set(event.value()),
            },
            input {
                class: "input",
                placeholder: "Enter commit message",
                value: commit_message(),
                oninput: move |event| commit_message.set(event.value()),
            },
            button {
                class: "button",
                onclick: move |_| {

                    if Url::parse(&repo_link().clone()).is_err() {
                        println!("Invalid URL: {}", repo_link().clone());
                        return;
                    }

                    spawn(async move {
                        if let Err(e) = git_helper::setup_git(&repo_link().clone()) {
                            println!("Git set repo failed: {}", e);
                        }
                    });
                    println!("{}", repo_link)
                },
                "Set Repo"
            },
            button {
                class: "button",
                onclick: move |_| {
                    if commit_message.is_empty() {
                        println!("commit message empty");
                        return;
                    }

                    spawn(async move {
                        if let Err(e) = git_helper::push(&commit_message().clone()) {
                            println!("Git push failed: {}", e);
                        }
                    });
                },
                "Push"
            },
            button { class: "button", "Pull" }
            button { class: "button", "Commit" }
        }
    }
}