//Run with dx serve

use dioxus::{prelude::*};
use std::path::Path;
use std::process::Command;
use url::Url;


const MAIN_CSS: Asset = asset!("/assets/main.css");
const JETBRAINS_MONO: Asset = asset!("/assets/fonts/JetBrainsMono-Medium.ttf");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Style { r#type: "text/css",
            {
                format!(
                    "@font-face {{ font-family: 'JetBrainsMono'; src: url('{}') format('truetype'); font-weight: normal; font-style: normal; }}",
                    JETBRAINS_MONO,
                )
            }
        }

        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Menu {}
    }
}

#[component]
pub fn Menu() -> Element {
    let mut repo_link = use_signal(|| "".to_string());

    rsx! {
        div { id: "buttons",
            input {
                class: "input",
                placeholder: "Enter repo link",
                value: "{repo_link()}",
                oninput: move |event| repo_link.set(event.value()),
            },
            button {
                class: "button",
                onclick: move |_| {

                    if Url::parse(&repo_link().clone()).is_err() {
                        println!("Invalid URL: {}", repo_link().clone());
                        return;
                    }

                    spawn(async move {
                        if let Err(e) = setup_git(&repo_link().clone()) {
                            println!("Git push failed: {}", e);
                        }
                    });
                    println!("{}", repo_link)
                },
                "Set Repo"
            },
            button {
                class: "button",
                onclick: move |_| {
                    spawn(async {
                        if let Err(e) = push() {
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

//Helper functions

fn run(cmd: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(cmd).args(args).status().map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("Command failed: {} {:?}", cmd, args));
    }
    Ok(())
}

fn current_branch() -> String {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("Failed to get current branch");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

//Main functions

fn setup_git(repo_link: &str) -> Result<(), String> {
    run("git", &["init"])?;

    let output = Command::new("git").args(["remote", "-v"]).output().map_err(|e| e.to_string())?;
    let remotes = String::from_utf8_lossy(&output.stdout);

    if remotes.lines().any(|line| line.starts_with("origin")) {
        let origin_line = remotes.lines().find(|line| line.starts_with("origin")).unwrap();
        let parts: Vec<&str> = origin_line.split_whitespace().collect();
        let current_url = parts.get(1).unwrap_or(&"<unknown>");

        println!("OVERWRITING: Remote origin' already exists: {}", current_url);
        run("git", &["remote", "set-url", "origin", repo_link])?;
        println!("Remote 'origin' updated to {}", repo_link);
    } else {
        run("git", &["remote", "add", "origin", repo_link])?;
        println!("ORIGIN SET TO: {}", repo_link);
    }

    Ok(())
}

fn push() -> Result<(), String> {
    if !Path::new(".git").exists() {
        return Err("No .git directory found".into());
    }

    run("git", &["add", "."])?;
    let _ = run("git", &["commit", "-m", "Test"]);

    let branch = current_branch(); // detect current branch
    println!("PUSHING TO BRANCH: {}", branch);
    run("git", &["push", "-u", "origin", &branch])?;

    Ok(())
}