//! Types for error handling.

use thiserror::Error;

/// SessionStoreError contains all the possible errors that can be returned
/// from [SessionStore](crate::store::SessionStore) operations.
#[derive(Error, Debug)]
pub enum SessionStoreError<'a> {
    /// Indicates a failure in loading the session from the
    /// [SessionStore](crate::store::SessionStore).
    #[error("failed to load session, `{0}`")]
    LoadFailure(&'a str),

    /// Indicates a failure in storing the session into the
    /// [SessionStore](crate::store::SessionStore).
    #[error("failed to store session, `{0}`")]
    StoreFailure(&'a str),

    /// Indicates a failure to clear the data in a session stored inside the
    /// [SessionStore](crate::store::SessionStore). Typically this is because
    /// the inner type was unable to reset itself to its default value.
    #[error("failed to clear session, `{0}`")]
    ClearFailure(&'a str),

    /// Indicates a failure to destroy a session stored inside the
    /// [SessionStore](crate::store::SessionStore).
    #[error("failed to destroy session, `{0}`")]
    DestroyFailure(&'a str),

    /// Indicates any other unspecified error that may be encountered when
    /// operating on a [SessionStore](crate::store::SessionStore)
    /// implementation.
    #[error("an unknown session store error occurred")]
    Unknown,
}
