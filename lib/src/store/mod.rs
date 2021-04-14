//! Traits and types for storing sessions during the
//! [Rocket](https://rocket.rs) application lifecycle.

use anyhow::Result;
use async_trait::async_trait;

use crate::{session::Session, SessionId};

pub mod error;
pub mod in_memory;

/// Trait implemented by types that store sessions.
/// This trait exposes a limited set of methods to do basic CRUD operations on
/// a collection of [`Session`]s.
///
/// ### Async Trait
///
/// This is an _async_ trait. Implementations must be decorated with `#[rocket::async_trait]`.
///
/// ### Methods
///
/// There are six total methods that are required by an implementor of this
/// trait. They provide the building blocks for any session store.
///
/// ### Example
///
/// Suppose you wanted to implement your own in-memory `SessionStore`, this is how it may look:
/// ```rust,no_run
/// todo!()
/// ```
#[async_trait]
pub trait SessionStore: Send + Sync + 'static {
    type SessionData: Clone + Default + Send + Sync + 'static;

    /// Initializes a new SessionStore
    ///
    /// Any necessary setup (i.e. migrating database tables, etc) should happen
    /// in this method.
    fn init() -> Self
    where
        Self: Sized;

    /// Loads a session from the provided store
    async fn load(&self, session_id: &SessionId) -> Result<Option<Session<Self::SessionData>>>;

    /// Stores the provided session into the store
    async fn store(&self, session: Session<Self::SessionData>) -> Result<()>;

    /// Removes all sessions, regardless of validity, from the session store
    async fn clear(&self) -> Result<()>;

    /// Completely removes the provided session from the store
    async fn destroy(&self, session_id: &SessionId) -> Result<()>;

    /// Removes all expired sessions from the session store
    async fn tidy(&self) -> Result<()>;
}
