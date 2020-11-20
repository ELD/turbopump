use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

use std::sync::{Arc, RwLock};

use crate::{util, CsrfToken, SessionID};

#[derive(Debug, Default)]
pub struct Session<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    id: SessionID,
    token: CsrfToken,
    should_destroy: bool,

    inner_data: Arc<RwLock<Data>>,
}

impl<Data> Clone for Session<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            token: self.token.clone(),
            inner_data: self.inner_data.clone(),

            should_destroy: false,
        }
    }
}

impl<Data> Session<Data>
where
    Data: Clone + Default + Send + Sync + 'static,
{
    pub fn new() -> Self {
        let id = util::random_string();
        let token = util::random_string();

        Self {
            id: SessionID(id),
            token: CsrfToken(token),
            inner_data: Default::default(),
            should_destroy: false,
        }
    }

    pub fn id(&self) -> &SessionID {
        &self.id
    }

    pub fn csrf_token(&self) -> &CsrfToken {
        &self.token
    }

    pub fn cookie_value(&self) -> (&str, &SessionID) {
        ("session_id", self.id())
    }

    pub fn token_value(&self) -> (&str, &CsrfToken) {
        ("xsrf_token", self.csrf_token())
    }

    pub fn should_destroy(&self) -> bool {
        self.should_destroy
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
        Outcome::Success(request.local_cache(Session::new))
    }
}

#[cfg(test)]
mod test {
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
}
