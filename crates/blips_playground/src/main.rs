use std::env;

use anyhow::Result;
use blips::{BlipsClient, CsrfToken, SessionCookie};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let session_cookie = SessionCookie::from(env::var("BLIPS_SESSION_COOKIE")?);
    let csrf_token = CsrfToken::from(env::var("BLIPS_CSRF_TOKEN")?);

    let client = BlipsClient::new(&session_cookie, &csrf_token);

    let response = client.me(blips::graphql::me::Variables {}).await?;

    println!("{:?}", response.me);

    Ok(())
}
