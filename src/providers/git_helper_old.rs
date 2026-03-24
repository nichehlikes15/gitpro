use git2::{
    Cred, ErrorCode, IndexAddOption, PushOptions, RemoteCallbacks, Repository, Signature,
};
use std::{error::Error, fs, path::Path};
use serde::Deserialize;

fn open_repo() -> Result<Repository, String> {
    Repository::discover(".").map_err(|e| e.message().to_string())
}

fn fallback_signature(repo: &Repository) -> Result<Signature<'_>, String> {
    repo.signature()
        .or_else(|_| Signature::now("gitpro", "gitpro@local"))
        .map_err(|e| e.message().to_string())
}

pub(crate) fn current_branch() -> String {
    let Ok(repo) = open_repo() else {
        return "No repository".to_string();
    };

    let result = match repo.head() {
        Ok(head) => head
            .shorthand()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "DETACHED_HEAD".to_string()),
        Err(_) => "No branch".to_string(),
    };

    result
}

pub(crate) fn current_repo() -> String {
    let Ok(repo) = open_repo() else {
        return "No repository".to_string();
    };

    let result = match repo.find_remote("origin") {
        Ok(remote) => remote
            .url()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "origin has no URL".to_string()),
        Err(_) => "No remote origin".to_string(),
    };

    result
}

fn normalize_repo_link(repo_link: &str) -> String {
    let trimmed = repo_link.trim();

    if let Some(rest) = trimmed.strip_prefix("https://github.com/") {
        let without_git = rest.strip_suffix(".git").unwrap_or(rest);
        let without_slash = without_git.trim_end_matches('/');
        return format!("git@github.com:{without_slash}.git");
    }

    if let Some(rest) = trimmed.strip_prefix("http://github.com/") {
        let without_git = rest.strip_suffix(".git").unwrap_or(rest);
        let without_slash = without_git.trim_end_matches('/');
        return format!("git@github.com:{without_slash}.git");
    }

    trimmed.to_string()
}

pub(crate) fn origin_url() -> Option<String> {
    let repo = open_repo().ok()?;
    let remote = repo.find_remote("origin").ok()?;
    remote.url().map(|s| s.to_string())
}

pub(crate) fn setup_git(repo_link: &str) -> Result<(), String> {
    let normalized = normalize_repo_link(repo_link);
    println!("setup_git: {}", normalized);
    let repo = open_repo().or_else(|_| Repository::init(".").map_err(|e| e.message().to_string()))?;

    let result = match repo.find_remote("origin") {
        Ok(_) => repo
            .remote_set_url("origin", &normalized)
            .map_err(|e| e.message().to_string()),
        Err(e) if e.code() == ErrorCode::NotFound => {
            repo.remote("origin", &normalized)
                .map(|_| ())
                .map_err(|e| e.message().to_string())
        }
        Err(e) => Err(e.message().to_string()),
    };

    result
}

#[derive(Deserialize)]
struct TokenData {
    token: String
}

fn get_token() -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string("token.json")?;
    let token_data: TokenData = serde_json::from_str(&data)?;

    Ok(token_data.token)
}

pub(crate) fn push(commit_message: &str) -> Result<(), String> {
    let repo = open_repo()?;

    println!("1");

    // Stage all tracked + untracked changes.
    let mut index = repo.index().map_err(|e| e.message().to_string())?;
    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .map_err(|e| e.message().to_string())?;
    index.write().map_err(|e| e.message().to_string())?;

    println!("2");

    let tree_id = index.write_tree().map_err(|e| e.message().to_string())?;
    let tree = repo.find_tree(tree_id).map_err(|e| e.message().to_string())?;

    // Create commit only when staged tree differs from HEAD tree.
    let mut should_commit = true;
    let mut parents = Vec::new();
    println!("3");

    if let Ok(head) = repo.head() {
        if let Some(target) = head.target() {
            let parent = repo.find_commit(target).map_err(|e| e.message().to_string())?;
            let parent_tree = parent.tree().map_err(|e| e.message().to_string())?;
            if parent_tree.id() == tree_id {
                should_commit = false;
            }
            parents.push(parent);
        }
    }
    println!("4");

    if should_commit {
        let sig = fallback_signature(&repo)?;
        let parent_refs: Vec<_> = parents.iter().collect();
        repo.commit(Some("HEAD"), &sig, &sig, commit_message, &tree, &parent_refs)
            .map_err(|e| e.message().to_string())?;
    }
    println!("5");

    let branch = repo
        .head()
        .ok()
        .and_then(|h| h.shorthand().map(|s| s.to_string()))
        .filter(|b| !b.is_empty())
        .unwrap_or_else(|| "main".to_string());
    println!("6");
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|url, username_from_url, _allowed_types| {
        let username = username_from_url.unwrap_or("git");

        // Prefer explicit SSH key for GitHub SSH URLs if configured
        if url.starts_with("git@github.com:") || url.starts_with("ssh://git@github.com") {
            if let Ok(key_path) = std::env::var("GITPRO_SSH_KEY_PATH") {
                let passphrase = std::env::var("GITPRO_SSH_PASSPHRASE").ok();
                return Cred::ssh_key(
                    username,
                    None,
                    std::path::Path::new(&key_path),
                    passphrase.as_deref(),
                );
            }

            return Cred::ssh_key_from_agent(username);
        }

        println!("7");
        // HTTPS GitHub fallback via PAT
        if url.contains("github.com") {
            if let Ok(token) = get_token() {
                if !token.trim().is_empty() {
                    return Cred::userpass_plaintext(&token, "")
                }
            }
        }
        println!("8.1");

        Cred::ssh_key_from_agent(username)
            .or_else(|_| {
                let cfg = repo.config()?;
                Cred::credential_helper(&cfg, url, username_from_url)
            })
            .or_else(|_| Cred::default())
    });
    println!("8");

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);
    println!("9");

    let mut remote = repo
        .find_remote("origin")
        .map_err(|_| "Remote 'origin' not found. Set repo first.".to_string())?;
    println!("10");

    let refspec = format!("refs/heads/{0}:refs/heads/{0}", branch);
    remote
        .push(&[&refspec], Some(&mut push_options))
        .map_err(|e| format!("Push failed: {}", e.message()))
}
