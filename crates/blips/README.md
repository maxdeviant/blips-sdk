# blips

An API client for [Blips](https://blips.app/).

## Prerequisites

You need to have a Blips account in order to authenticate with the Blips API.

Currently signups are restricted, but you can join the waitlist.

## Authentication

Blips does not currently have a dedicated API authentication mechanism, so you'll need to sign in to the Blips web app and grab your Blips session token.

You will need the value from the `Cookie` header, which will look something like this:

```text
user_session=<USER_SESSION_VALUE>; _chex_session=<CHEX_SESSION_VALUE>
```

You will also need the value of the `X-CSRF-Token` header.

Provide both of these values when constructing the `BlipsClient`:

```rs
let session_cookie = SessionCookie::from(env::var("BLIPS_SESSION_COOKIE")?);
let csrf_token = CsrfToken::from(env::var("BLIPS_CSRF_TOKEN")?);

let client = BlipsClient::new(&session_cookie, &csrf_token);
```

## Usage

Once you've constructed a `BlipsClient` you can make requests to the Blips API using the methods on the client:

```rs
let me = client
    .me(blips::graphql::me::Variables {})
    .await?
    .me
    .ok_or_else(|| anyhow!("Failed to execute `me`"))?;

println!("Signed in as {}", me.email);
```
