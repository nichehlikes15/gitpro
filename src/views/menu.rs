use dioxus::{prelude::*};

use url::Url;

use crate::components::topbar;
use crate::providers::git_helper;

#[component]
pub fn Menu() -> Element {
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
            // button {
            //     class: "button",
            //     onclick: move |_| {
            //         let input = repo_link().trim().to_string();

            //         let url_to_use = if input.is_empty() {
            //             match git_helper::origin_url() {
            //                 Some(existing) => existing,
            //                 None => {
            //                     println!("No existing Git remote to use. Please enter a repository URL.");
            //                     return;
            //                 }
            //             }
            //         } else {
            //             if Url::parse(&input).is_err() {
            //                 println!("Invalid URL: {}", input);
            //                 return;
            //             }
            //             input
            //         };

            //         let url_for_task = url_to_use.clone();
            //         println!("{}", url_to_use);

            //         spawn(async move {
            //             if let Err(e) = git_helper::setup_git(&url_for_task) {
            //                 println!("Git set repo failed: {}", e);
            //             }
            //         });
            //     },
            //     "Set Repo"
            // },
            button {
                class: "button",
                onclick: move |_| {
                    if commit_message().trim().is_empty() {
                        println!("commit message empty");
                        return;
                    }

                    spawn(async move {
                        match git_helper::push(&commit_message().clone()) {
                            Ok(_) => println!("Git push succeeded."),
                            Err(e) => println!("Git push failed: {}", e),
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