//! Types for error handling.

use thiserror::Error;

/// SessionStoreError contains all the possible errors that can be returned
/// from [crate::store::SessionStore] operations.
#[derive(Error, Debug)]
pub enum SessionStoreError {
    #[error("failed to load session, `{0}`")]
    LoadFailure(String),
    #[error("failed to store session, `{0}`")]
    StoreFailure(String),
    #[error("failed to clear session, `{0}`")]
    ClearFailure(String),
    #[error("failed to destroy session, `{0}`")]
    DestroyFailure(String),
    #[error("an unknown session store error occurred")]
    Unknown,
}
