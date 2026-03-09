use octocrab::Octocrab;
use std::error::Error;

pub(crate) async fn get_username_api(token: &str) -> Result<String, Box<dyn Error>> {
    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let user = octocrab.current().user().await?;

    Ok(user.login)
}

pub(crate) async fn star_repo(token: &str) -> Result<(), Box<dyn Error>> {
    let octocrab = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    octocrab
        .put::<(), _, _>(
            "/user/starred/nichehlikes15/gitpro",
            Some(&()),
        )
        .await?;

    println!("Starred nichehlikes15/gitpro");

    Ok(())
}