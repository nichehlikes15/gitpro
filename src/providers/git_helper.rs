use git2::{
    Cred, ErrorCode, IndexAddOption, PushOptions, RemoteCallbacks, Repository, Signature,
};

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

pub(crate) fn setup_git(repo_link: &str) -> Result<(), String> {
    let repo = open_repo().or_else(|_| Repository::init(".").map_err(|e| e.message().to_string()))?;

    let result = match repo.find_remote("origin") {
        Ok(_) => repo
            .remote_set_url("origin", repo_link)
            .map_err(|e| e.message().to_string()),
        Err(e) if e.code() == ErrorCode::NotFound => {
            repo.remote("origin", repo_link)
                .map(|_| ())
                .map_err(|e| e.message().to_string())
        }
        Err(e) => Err(e.message().to_string()),
    };

    result
}

pub(crate) fn push(commit_message: &str) -> Result<(), String> {
    let repo = open_repo()?;

    // Stage all tracked + untracked changes.
    let mut index = repo.index().map_err(|e| e.message().to_string())?;
    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .map_err(|e| e.message().to_string())?;
    index.write().map_err(|e| e.message().to_string())?;

    let tree_id = index.write_tree().map_err(|e| e.message().to_string())?;
    let tree = repo.find_tree(tree_id).map_err(|e| e.message().to_string())?;

    // Create commit only when staged tree differs from HEAD tree.
    let mut should_commit = true;
    let mut parents = Vec::new();

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

    if should_commit {
        let sig = fallback_signature(&repo)?;
        let parent_refs: Vec<_> = parents.iter().collect();
        repo.commit(Some("HEAD"), &sig, &sig, commit_message, &tree, &parent_refs)
            .map_err(|e| e.message().to_string())?;
    }

    let branch = repo
        .head()
        .ok()
        .and_then(|h| h.shorthand().map(|s| s.to_string()))
        .filter(|b| !b.is_empty())
        .unwrap_or_else(|| "main".to_string());

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
            .or_else(|_| {
                let cfg = repo.config()?;
                Cred::credential_helper(&cfg, url, username_from_url)
            })
            .or_else(|_| Cred::default())
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    let mut remote = repo
        .find_remote("origin")
        .map_err(|_| "Remote 'origin' not found. Set repo first.".to_string())?;

    let refspec = format!("refs/heads/{0}:refs/heads/{0}", branch);
    remote
        .push(&[&refspec], Some(&mut push_options))
        .map_err(|e| format!("Push failed: {}", e.message()))
}
