use anyhow::Result;
use async_trait::async_trait;

use crate::{session::Session, SessionID};

pub mod in_memory;

#[async_trait]
pub trait SessionStore: Send + Sync + 'static {
    type SessionData: Clone + Default + Send + Sync + 'static;

    fn init() -> Self
    where
        Self: Sized;
    async fn load(&self, session_id: SessionID) -> Result<Option<Session<Self::SessionData>>>;
    async fn store(&self, session: Session<Self::SessionData>) -> Result<()>;
    async fn clear(&self) -> Result<()>;
    async fn destroy(&self, session: Session<Self::SessionData>) -> Result<()>;
}
