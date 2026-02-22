use git2::Repository;

pub(crate) fn github_username_or_not_logged_in() -> String {
    let repo = match Repository::discover(".") {
        Ok(r) => r,
        Err(_) => return "Not Logged In".to_string(),
    };

    if let Ok(cfg) = repo.config() {
        if let Ok(name) = cfg.get_string("credential.username") {
            if !name.trim().is_empty() {
                return name;
            }
        }

        if let Ok(name) = cfg.get_string("user.name") {
            if !name.trim().is_empty() {
                return name;
            }
        }
    }

    if let Ok(remote) = repo.find_remote("origin") {
        if let Some(url) = remote.url() {
            if let Some(user) = parse_github_user_from_remote(url) {
                return user;
            }
        }
    }

    "Not Logged In".to_string()
}

fn parse_github_user_from_remote(url: &str) -> Option<String> {
    if let Some(rest) = url.strip_prefix("https://github.com/") {
        let user = rest.split('/').next()?.trim();
        if !user.is_empty() {
            return Some(user.to_string());
        }
    }

    if let Some(rest) = url.strip_prefix("git@github.com:") {
        let user = rest.split('/').next()?.trim();
        if !user.is_empty() {
            return Some(user.to_string());
        }
    }

    None
}