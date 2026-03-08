use octocrab::Octocrab;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Email {
    email: String,
    primary: bool,
    verified: bool,
    visibility: Option<String>,
}

pub(crate) async fn get_username_api(token: &str) -> Result<String, Box<dyn Error>> {
    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let user = octocrab.current().user().await?;

    //println!("Username: {}", user.login);
    //println!("Public Email: {:?}", user.email);

    // Fetch emails via the API
    //let emails: Vec<Email> = octocrab
    //    .get("/user/emails", None::<&()>)
    //    .await?;

    //println!("\nEmails:");
    //for email in emails {
    //    println!(
    //        "- {} | primary: {} | verified: {} | visibility: {:?}",
    //        email.email, email.primary, email.verified, email.visibility
    //    );
    //}

    Ok(user.login)
}

pub(crate) async fn star_repo(token: &str) -> Result<(), Box<dyn Error>> {
    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    // Empty request body (Some(&())) and ignore response with ()
    octocrab
        .put::<(), (), _>(
            "/user/starred/nichehlikes15/gitpro",
            Some(&()),
        )
        .await?;

    println!("Starred nichehlikes15/gitpro");

    Ok(())
}