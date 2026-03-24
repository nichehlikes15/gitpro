use octocrab::Octocrab;
use serde::Deserialize;
use std::{error::Error, fs, path::Path};

#[derive(Deserialize)]
struct TokenData {
    token: String,
    username: String,
}

pub(crate) async fn check_token() -> Result<bool, Box<dyn Error>> {
    if !Path::new("token.json").exists() {
        return Ok(false);
    }

    let token = get_token().await?;

    match get_username_api(&token).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

async fn get_username_api(token: &str) -> Result<String, Box<dyn Error>> {
    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let user = octocrab.current().user().await?;
    Ok(user.login)
}

async fn get_token() -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string("token.json")?;
    let token_data: TokenData = serde_json::from_str(&data)?;

    Ok(token_data.token)
}
