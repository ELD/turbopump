use anyhow::Result;
use flurry::HashMap;
use std::sync::Arc;

use crate::{
    session::Session,
    store::{error::SessionStoreError, SessionStore},
    SessionId,
};

#[derive(Clone)]
pub struct InMemory<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    sessions: Arc<HashMap<SessionId, Session<Data>>>,
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

    async fn load(&self, session_id: &SessionId) -> Result<Option<Session<Self::SessionData>>> {
        let sessions_ref = self.sessions.pin();
        let session = sessions_ref
            .get(session_id)
            .cloned()
            .and_then(Session::validate);

        Ok(session)
    }

    async fn store(&self, session: Session<Self::SessionData>) -> Result<()> {
        let sessions_ref = self.sessions.pin();
        sessions_ref.insert(session.id().clone(), session);

        Ok(())
    }

    async fn clear(&self) -> Result<()> {
        let session_ref = self.sessions.pin();
        session_ref.clear();

        Ok(())
    }

    async fn destroy(&self, session_id: &SessionId) -> Result<()> {
        let sessions_ref = self.sessions.pin();
        sessions_ref
            .remove(session_id)
            .map(|_| ())
            .ok_or(SessionStoreError::DestroyFailure(
                "unable to destroy session",
            ))?;

        Ok(())
    }

    async fn tidy(&self) -> Result<()> {
        self.sessions.pin().retain(|_, sess| sess.is_valid());

        Ok(())
    }
}

#[cfg(test)]
impl<Data> InMemory<Data>
where
    Data: Clone + Default + Send + Sync,
{
    fn count(&self) -> usize {
        self.sessions.len()
    }
}

#[cfg(test)]
mod test {
    use super::{InMemory, SessionStore};
    use crate::Session;
    use rocket::tokio;

    #[derive(Clone, Default)]
    struct SessionData {
        dummy: usize,
    }

    #[tokio::test]
    async fn creating_a_new_session_with_default_expiry() {
        let store = InMemory::<SessionData>::init();
        let session = Session::<SessionData>::new(3600);

        store.store(session.clone()).await.unwrap();

        let loaded_session = store.load(session.id()).await.unwrap().unwrap();
        assert_eq!(session.id(), loaded_session.id());
        assert!(!loaded_session.expired());
        assert!(loaded_session.is_valid());
    }

    #[tokio::test]
    async fn loading_an_expired_session_gives_a_new_one() {
        let store = InMemory::<SessionData>::init();
        let session = Session::<SessionData>::new(0);

        store.store(session.clone()).await.unwrap();

        let loaded_session = store.load(session.id()).await.unwrap();
        assert!(loaded_session.is_none());
    }

    #[tokio::test]
    async fn updating_a_session() {
        let store = InMemory::<SessionData>::init();
        let session = Session::<SessionData>::new(3600);

        store.store(session.clone()).await.unwrap();

        let stored_session = store.load(session.id()).await.unwrap().unwrap();
        stored_session.tap(|sess| {
            sess.dummy = 1000;
        });

        assert!(store.store(stored_session).await.is_ok());
        let loaded_session = store.load(session.id()).await.unwrap().unwrap();
        assert_eq!(loaded_session.tap(|sess| sess.dummy), 1000);
    }

    #[tokio::test]
    async fn extending_expiry() {
        let store = InMemory::<SessionData>::init();
        let session = Session::<SessionData>::new(3600);
        let original_expiration = session.expiration();

        store.store(session.clone()).await.unwrap();
        let mut stored_session = store.load(session.id()).await.unwrap().unwrap();
        stored_session.renew(3600);
        store.store(stored_session).await.unwrap();

        let loaded_expiration = store
            .load(session.id())
            .await
            .unwrap()
            .unwrap()
            .expiration();
        assert!(original_expiration != loaded_expiration);
    }

    #[tokio::test]
    async fn destroying_a_session() {
        let store = InMemory::<SessionData>::init();
        for _ in 0..3 {
            store
                .store(Session::<SessionData>::new(3600))
                .await
                .unwrap();
        }

        let session = Session::<SessionData>::new(3600);
        store.store(session.clone()).await.unwrap();
        assert_eq!(4, store.count());

        store.destroy(session.id()).await.unwrap();
        assert_eq!(3, store.count());
        assert!(store.load(session.id()).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn clearing_store() {
        let store = InMemory::<SessionData>::init();
        for _ in 0..3 {
            store
                .store(Session::<SessionData>::new(3600))
                .await
                .unwrap();
        }

        assert_eq!(3, store.count());
        store.clear().await.unwrap();
        assert_eq!(0, store.count());
    }

    #[tokio::test]
    async fn tidying_store_clears_expired_sessions() {
        let store = InMemory::<SessionData>::init();
        for _ in 0..3 {
            store.store(Session::<SessionData>::new(0)).await.unwrap();
        }

        let session = Session::<SessionData>::new(3600);
        store.store(session.clone()).await.unwrap();

        assert_eq!(4, store.count());
        store.tidy().await.unwrap();
        assert_eq!(1, store.count());
    }
}
