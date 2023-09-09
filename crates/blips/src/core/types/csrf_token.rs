use std::fmt::Display;

/// A Blips CSRF token.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CsrfToken(String);

impl Display for CsrfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for CsrfToken {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for CsrfToken {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
