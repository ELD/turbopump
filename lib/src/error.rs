use thiserror::Error;

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
