use octocrab::Octocrab;
use serde::Deserialize;
use tokio;

#[derive(Debug, Deserialize)]
struct Email {
    email: String,
    primary: bool,
    verified: bool,
    visibility: Option<String>,
}

#[tokio::main]
pub(crate) async fn get_username_api(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let user = octocrab.current().user().await?;

    println!("Username: {}", user.login);
    println!("Name: {:?}", user.name);
    println!("Public Email: {:?}", user.email);

    // Fetch emails via the API
    let emails: Vec<Email> = octocrab
        .get("/user/emails", None::<&()>)
        .await?;

    println!("\nEmails:");
    for email in emails {
        println!(
            "- {} | primary: {} | verified: {} | visibility: {:?}",
            email.email, email.primary, email.verified, email.visibility
        );
    }

    Ok(())
}