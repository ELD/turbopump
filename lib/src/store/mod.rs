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
/// ### Example
///
/// Suppose you wanted to implement your own in-memory `SessionStore`, this is how it may look:
/// ```rust
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

    async fn load(&self, session_id: &SessionId) -> Result<Option<Session<Self::SessionData>>>;

    async fn store(&self, session: Session<Self::SessionData>) -> Result<()>;

    async fn clear(&self) -> Result<()>;

    async fn destroy(&self, session_id: &SessionId) -> Result<()>;

    async fn tidy(&self) -> Result<()>;
}
