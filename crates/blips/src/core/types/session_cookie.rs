use std::fmt::Display;

/// A Blips session cookie.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionCookie(String);

impl Display for SessionCookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SessionCookie {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for SessionCookie {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
