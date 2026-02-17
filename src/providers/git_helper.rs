use std::path::Path;
use std::process::Command;
use git2::{Cred, IndexAddOption, PushOptions, RemoteCallbacks, Repository,};

fn run(cmd: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(cmd).args(args).status().map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("Command failed: {} {:?}", cmd, args));
    }
    Ok(())
}

pub(crate) fn current_branch() -> String {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .expect("Failed to get current branch");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub(crate) fn current_repo() -> String {
    let output = Command::new("git")
        .args(["remote", "-v"])
        .output()
        .expect("Failed to get current branch");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

//Main functions

pub(crate) fn setup_git(repo_link: &str) -> Result<(), String> {
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

pub(crate) fn push(commit_message: &str) -> Result<(), String> {
    if !Path::new(".git").exists() {
        return Err("No .git directory found".into());
    }
    // open the git repository
    let repo = Repository::open(".")
        .map_err(|e| e.to_string())?;

    // command: git add .
    let mut index = repo.index()
        .map_err(|e| e.to_string())?;

    // stage all files
    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .map_err(|e| e.to_string())?;

    // write staged
    index.write()
        .map_err(|e| e.to_string())?;

    // create a tree from the index for commit
    let tree_id = index.write_tree()
        .map_err(|e| e.to_string())?;

    let tree = repo.find_tree(tree_id)
        .map_err(|e| e.to_string())?;

    // command: git commit -m commit_message
    let signature = repo.signature()
        .map_err(|e| e.to_string())?;

    // get current HEAD commit (if any)
    let parent_commit = repo
        .head()
        .ok()
        .and_then(|h| h.target())
        .and_then(|oid| repo.find_commit(oid).ok());

    let commit_result = match parent_commit {
        // normal commit (repo already has commits)
        Some(parent) => repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            commit_message,
            &tree,
            &[&parent],
        ),

        // first commit (no parents)
        None => repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            commit_message,
            &tree,
            &[],
        ),
    };

    // ignore "nothing to commit" errors
    // replaces: let _ = run("git", ["commit", ...])
    let _ = commit_result;

    // command: current_branch()
    // (same as: git branch --show-current)
    let head = repo.head()
        .map_err(|e| e.to_string())?;

    let branch = head
        .shorthand()
        .ok_or("Failed to detect current branch")?
        .to_string();

    println!(
        "PUSHING TO BRANCH: {} WITH COMMIT MESSAGE: {}",
        branch, commit_message
    );

    // =========================================================
    // authentication (git CLI does this implicitly)
    // =========================================================
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username, _| {
        // uses ssh-agent (same behavior as git push via SSH)
        Cred::ssh_key_from_agent(username.unwrap())
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    // =========================================================
    // replaces: git push -u origin <branch>
    // =========================================================
    let mut remote = repo
        .find_remote("origin")
        .map_err(|e| e.to_string())?;

    // local branch -> remote branch
    remote
        .push(
            &[format!("refs/heads/{}:refs/heads/{}", branch, branch)],
            Some(&mut push_options),
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}