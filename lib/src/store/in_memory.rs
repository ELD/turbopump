use anyhow::Result;
use flurry::HashMap;
use std::sync::Arc;

use crate::{error::SessionStoreError, session::Session, store::SessionStore, SessionID};

#[derive(Clone)]
pub struct InMemory<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    sessions: Arc<HashMap<SessionID, Session<Data>>>,
}

#[async_trait::async_trait]
impl<Data> SessionStore for InMemory<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    type SessionData = Data;

    fn init() -> Self {
        Self {
            sessions: Default::default(),
        }
    }

    async fn load(&self, session_id: SessionID) -> Result<Option<Session<Self::SessionData>>> {
        let sessions_ref = self.sessions.pin();
        let session = sessions_ref.get(&session_id).cloned();

        Ok(session)
    }

    async fn store(&self, session: Session<Self::SessionData>) -> Result<()> {
        let sessions_ref = self.sessions.pin();
        sessions_ref.insert(session.id().clone(), session);

        Ok(())
    }

    async fn clear(&self) -> Result<()> {
        todo!()
    }

    async fn destroy(&self, session: Session<Self::SessionData>) -> Result<()> {
        if session.should_destroy() {
            let sessions_ref = self.sessions.pin();
            sessions_ref
                .remove(session.id())
                .map(|_| ())
                .ok_or_else(|| {
                    SessionStoreError::DestroyFailure("unable to destroy session".to_string())
                })?
        }

        Err(SessionStoreError::Unknown.into())
    }
}
