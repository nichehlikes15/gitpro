use serde::{Serialize, Deserialize};
use std::fs;

use crate::providers::login::webserver;
use crate::providers::login::api;

#[derive(Serialize, Deserialize)]
struct Data {
    token: String,
}

pub async fn start_webserver() -> String {
    let token = webserver::start().await;
    let username = api::get_username_api(&token);


    let data = Data {
        token: token.clone(),
    };

    let json = serde_json::to_string_pretty(&data).unwrap();
    fs::write("src/token.json", json).unwrap();

    token
}