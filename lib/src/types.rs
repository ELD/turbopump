use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CsrfToken(pub String);

impl Display for CsrfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Default, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionID(pub String);

impl Display for SessionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<SessionID> for &str {
    fn into(self) -> SessionID {
        SessionID(self.to_string())
    }
}
