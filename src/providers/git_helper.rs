use dioxus::Ok;
use octocrab::Octocrab;
use serde::Deserialize;

// pub(crate) async fn commit(token: &str, owner: &str, repo: &str, branch: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let octocrab = Octocrab::builder()
//         .personal_token(token.to_string())
//         .build()?;


//     Ok(())
// }