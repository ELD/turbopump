//! Core type for expressing sessions.

use chrono::{DateTime, Duration, Utc};
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

use std::sync::{Arc, RwLock};

use crate::{util, SessionId};

/// Type containing data for a given session.
///
/// The basic session type. It takes a generic `Data` type that is stored in
/// the wrapper. This allows for a custom data structure to be used and
/// manipulated in an ergonomic way via the [`Session::tap`] method.
#[derive(Debug)]
pub struct Session<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    id: SessionId,

    inner_data: Arc<RwLock<Data>>,

    expiration: DateTime<Utc>,
}

impl<Data> Clone for Session<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),

            inner_data: self.inner_data.clone(),

            expiration: self.expiration,
        }
    }
}

impl<Data> Default for Session<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            id: SessionId::default(),
            inner_data: Arc::new(RwLock::new(Data::default())),
            expiration: Utc::now(),
        }
    }
}

impl<Data> Session<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    pub fn new(lifespan: i64) -> Self {
        let id = util::random_string();

        Self {
            id: SessionId(id),
            inner_data: Default::default(),
            expiration: Utc::now()
                .checked_add_signed(Duration::seconds(lifespan))
                .unwrap_or_else(Utc::now),
        }
    }

    pub fn id(&self) -> &SessionId {
        &self.id
    }

    pub fn cookie_value(&self) -> (&str, &SessionId) {
        ("session_id", self.id())
    }

    pub fn renew(&mut self, lifespan: i64) {
        self.expiration = Utc::now()
            .checked_add_signed(Duration::seconds(lifespan))
            .unwrap_or_else(Utc::now)
    }

    pub fn expiration(&self) -> DateTime<Utc> {
        self.expiration
    }

    pub fn expired(&self) -> bool {
        self.expiration <= Utc::now()
    }

    pub fn is_valid(&self) -> bool {
        !self.expired()
    }

    pub fn validate(self) -> Option<Self> {
        if self.is_valid() {
            Some(self)
        } else {
            None
        }
    }

    pub fn tap<T>(&self, f: impl FnOnce(&mut Data) -> T) -> T {
        f(&mut self.inner_data.write().unwrap())
    }
}

#[async_trait::async_trait]
impl<'a, 'r, Data> FromRequest<'a, 'r> for &'a Session<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        Outcome::Success(request.local_cache(Session::default))
    }
}

#[cfg(test)]
mod test {
    use std::thread;

    use super::*;

    #[test]
    fn tap_mutates_session() {
        #[derive(Clone, Default)]
        struct Counter {
            count: u32,
        }

        let session = Session::<Counter>::default();
        assert_eq!(0, session.inner_data.read().unwrap().count);

        let count = session.tap(|counter| {
            counter.count += 1;
            counter.count
        });

        assert_eq!(1, count);
        assert_eq!(1, session.inner_data.read().unwrap().count);
    }

    #[test]
    fn sessions_expire() {
        #[derive(Clone, Default)]
        struct Data;
        let session = Session::<Data>::new(1);
        assert!(!session.expired());

        thread::sleep(std::time::Duration::from_secs(1));

        assert!(session.expired());
    }
}
