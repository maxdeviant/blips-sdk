use graphql_client::GraphQLQuery;
use url::{ParseError, Url};

use crate::{CsrfToken, SessionCookie};

/// The Blips client.
pub struct BlipsClient {
    base_url: Url,
    session_cookie: SessionCookie,
    csrf_token: CsrfToken,
    client: reqwest::Client,
}

impl BlipsClient {
    /// Returns a new instance of the Blips client using the provided session token.
    pub fn new(session_cookie: &SessionCookie, csrf_token: &CsrfToken) -> Self {
        BlipsClientBuilder::new(session_cookie, csrf_token).build()
    }

    /// Returns a [`BlipsClientBuilder`] that may be used to construct a Blips client.
    pub fn builder<'a>(
        session_cookie: &'a SessionCookie,
        csrf_token: &'a CsrfToken,
    ) -> BlipsClientBuilder<'a> {
        BlipsClientBuilder::new(session_cookie, csrf_token)
    }

    pub(crate) fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub(crate) fn session_cookie(&self) -> &SessionCookie {
        &self.session_cookie
    }

    pub(crate) fn csrf_token(&self) -> &CsrfToken {
        &self.csrf_token
    }

    pub(crate) async fn post_graphql<Q: GraphQLQuery>(
        &self,
        variables: Q::Variables,
    ) -> Result<graphql_client::Response<Q::ResponseData>, reqwest::Error> {
        let body = Q::build_query(variables);

        let response = self
            .client
            .post(self.base_url().clone())
            .header("Cookie", self.session_cookie().to_string())
            .header("X-Csrf-Token", self.csrf_token().to_string())
            .json(&body)
            .send()
            .await?;

        response.json().await
    }
}

/// A builder for a Blips client.
pub struct BlipsClientBuilder<'a> {
    base_url: Url,
    session_cookie: &'a SessionCookie,
    csrf_token: &'a CsrfToken,
}

impl<'a> BlipsClientBuilder<'a> {
    /// Returns a new [`BlipsClientBuilder`] using the provided session token.
    pub fn new(session_cookie: &'a SessionCookie, csrf_token: &'a CsrfToken) -> Self {
        Self {
            base_url: Url::parse("https://blips.app/query").unwrap(),
            session_cookie,
            csrf_token,
        }
    }

    /// Sets the base URL of the Blips API that the client should point to.
    pub fn base_url(mut self, base_url: &'a str) -> Result<BlipsClientBuilder, ParseError> {
        self.base_url = Url::parse(base_url)?;
        Ok(self)
    }

    /// Sets the session token that the client will use.
    pub fn session_cookie(mut self, session_cookie: &'a SessionCookie) -> Self {
        self.session_cookie = session_cookie;
        self
    }

    /// Sets the CSRF token that the client will use.
    pub fn csrf_token(mut self, csrf_token: &'a CsrfToken) -> Self {
        self.csrf_token = csrf_token;
        self
    }

    /// Consumes the builder and returns the constructed client.
    pub fn build(self) -> BlipsClient {
        let client = reqwest::Client::builder()
            .user_agent(concat!("blips/", env!("CARGO_PKG_VERSION")))
            .build()
            .unwrap();

        BlipsClient {
            base_url: self.base_url,
            session_cookie: self.session_cookie.to_owned(),
            csrf_token: self.csrf_token.to_owned(),
            client,
        }
    }
}
