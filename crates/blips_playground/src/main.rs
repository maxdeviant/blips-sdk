use std::env;

use anyhow::{anyhow, Result};
use blips::{BlipsClient, CsrfToken, SessionCookie};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let session_cookie = SessionCookie::from(env::var("BLIPS_SESSION_COOKIE")?);
    let csrf_token = CsrfToken::from(env::var("BLIPS_CSRF_TOKEN")?);

    let client = BlipsClient::new(&session_cookie, &csrf_token);

    let me = client
        .me(blips::graphql::me::Variables {})
        .await?
        .me
        .ok_or_else(|| anyhow!("Failed to execute `me`"))?;

    println!("Signed in as {}", me.email);

    let tasks = client
        .tasks(blips::graphql::tasks::Variables {
            completed: Some(true),
            focus: None,
            inbox: None,
            date: None,
            due_date: None,
            project_id: None,
        })
        .await?
        .tasks
        .ok_or_else(|| anyhow!("Failed to retrieve tasks"))?;

    println!("You have {} task(s):", tasks.len());

    for task in tasks {
        println!("[{}] {}", if task.completed { "x" } else { " " }, task.name);
    }

    Ok(())
}
