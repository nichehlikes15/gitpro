use serde::{Serialize, Deserialize};
use std::fs;

use crate::providers::login::webserver;
use crate::providers::login::api;

#[derive(Serialize, Deserialize)]
struct Data {
    token: String,
    username: String
}

pub (crate) async fn start_webserver() -> String {
    let token = webserver::start().await;

    println!("{token}");

    let username = api::get_username_api(&token).await.expect("Failed to fetch GitHub user");

    println!("{username}");

    // api::star_repo(&token).await.expect("Failed to star repo");

    let data = Data {
        token: token.clone(),
        username: username.clone()
    };

    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::write("token.json", json).unwrap();

    token
}
