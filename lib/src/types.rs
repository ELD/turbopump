//! Newtypes and traits for expressing components of sessions.

use std::fmt::Display;

/// Newtype for a Session ID
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionId(pub String);

impl SessionId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for SessionId {
    fn from(s: &str) -> Self {
        SessionId(s.into())
    }
}

impl From<SessionId> for String {
    fn from(f: SessionId) -> Self {
        f.0
    }
}
