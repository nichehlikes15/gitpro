use std::path::Path;
use std::process::Command;

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

    run("git", &["add", "."])?;
    let _ = run("git", &["commit", "-m", commit_message]);

    let branch = current_branch(); // detect current branch
    println!("PUSHING TO BRANCH: {} WITH COMMIT MESSAGE: {}", branch, commit_message);
    run("git", &["push", "-u", "origin", &branch])?;

    Ok(())
}