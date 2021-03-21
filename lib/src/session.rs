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
    /// Creates a new session object with the specified lifetime
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

    /// Returns the [`SessionId`]
    pub fn id(&self) -> &SessionId {
        &self.id
    }

    /// Returns a tuple containing the string, "session_id" and the Session's
    /// [`SessionId`]. This is useful for setting cookie values.
    pub fn cookie_value(&self) -> (&str, &SessionId) {
        ("session_id", self.id())
    }

    /// Renews the expiry of the session by the specified lifetime.
    pub fn renew(&mut self, lifespan: i64) {
        self.expiration = Utc::now()
            .checked_add_signed(Duration::seconds(lifespan))
            .unwrap_or_else(Utc::now)
    }

    /// Returns the expiration date time of the session.
    pub fn expiration(&self) -> DateTime<Utc> {
        self.expiration
    }

    /// Returns whether the session is expired or not
    pub fn expired(&self) -> bool {
        self.expiration <= Utc::now()
    }

    /// Checks if the session is still valid, i.e. unexpired.
    pub fn is_valid(&self) -> bool {
        !self.expired()
    }

    /// Checks whether the session is still valid
    pub fn validate(self) -> Option<Self> {
        if self.is_valid() {
            Some(self)
        } else {
            None
        }
    }

    /// Allows for ergonomically editing the inner data of the session object.
    ///
    /// You can also return aribtrary data from the closure passed into the
    /// this method.
    ///
    /// ### Example
    /// ```
    /// # use turbopump::Session;
    /// #[derive(Clone, Default)]
    /// struct SessionData {
    ///    hits: usize,
    /// }
    ///
    /// let session = Session::new(3600);
    /// let hits = session.tap(|data: &mut SessionData| {
    ///     data.hits += 1;
    ///     data.hits
    /// });
    /// #
    /// # assert_eq!(1, hits);
    /// ```
    pub fn tap<T>(&self, f: impl FnOnce(&mut Data) -> T) -> T {
        f(&mut self.inner_data.write().unwrap())
    }
}

#[async_trait::async_trait]
impl<'r, Data> FromRequest<'r> for &'r Session<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
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
