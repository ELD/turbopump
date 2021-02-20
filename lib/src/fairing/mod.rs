//! Types for interoperating with [Rocket](https://rocket.rs), and
//! configuration.
//!
//! This module contains the fairing implementation to allow for instantiating,
//! deleting, and accessing [Session](crate::session::Session)s.
//!
//! ## Attaching
//!
//! You must attach the fairing by calling [`Rocket::attach()`](Rocket::attach) on the
//! application's [`Rocket`](rocket::Rocket) instance. This also requires supplying the
//! [SessionStore](crate::store::SessionStore) instance, as well. For example:
//!
//! ```rust
//! # use rocket::{error::Error as RocketError, get, response::content::Html, routes};
//! # use turbopump::{
//! #     fairing::{config::SessionConfig, SessionFairing},
//! #     store::in_memory::InMemory,
//! #     Session,
//! # };
//! #
//! # #[derive(Clone, Default)]
//! # pub struct Data {
//! #     count: u32,
//! # }
//! #
//! rocket::ignite().attach(SessionFairing::<InMemory<Data>>::init());
//! ```
//!
//! ## Ordering
//! Because of order dependecy with fairings, it's advised that you attach
//! this fairing before others, as it fetches the session cookie from the
//! incoming request, attempts to retrieve an already-started session (or
//! starts one), and then inserts it into the request-local cache.

use std::marker::PhantomData;

use async_trait::async_trait;
use rand::Rng;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Cookie,
    Data, Request, Response, Rocket,
};

use crate::{
    fairing::config::SessionConfig,
    store::SessionStore,
    util::{cookie_value_exists, make_cookie},
    Session,
};

pub mod config;

/// The core fairing that enables session management for your application.
pub struct SessionFairing<Store: SessionStore> {
    config: SessionConfig,
    store: PhantomData<Store>,
}

impl<Store: SessionStore> SessionFairing<Store> {
    pub fn init() -> Self {
        let config = rocket::config::Config::figment()
            .extract_inner::<SessionConfig>("session")
            .unwrap_or_else(|_| SessionConfig::default());

        Self {
            store: PhantomData,
            config,
        }
    }

    pub fn with_config(config: SessionConfig) -> Self {
        Self {
            store: PhantomData,
            config,
        }
    }

    async fn init_session(
        &self,
        session_cookie: Option<&Cookie<'_>>,
        store: &Store,
        config: &SessionConfig,
    ) -> Session<<Store as SessionStore>::SessionData> {
        if let Some(cookie) = session_cookie {
            store
                .load(&cookie.value().into())
                .await
                .unwrap()
                .map(|mut sess| {
                    sess.renew(config.max_age);
                    sess
                })
                .unwrap_or_else(|| Session::new(config.max_age))
        } else {
            Session::new(config.max_age)
        }
    }
}

#[async_trait]
impl<Store: SessionStore> Fairing for SessionFairing<Store> {
    fn info(&self) -> Info {
        Info {
            name: "Turbopump (session management)",
            kind: Kind::Attach | Kind::Request | Kind::Response,
        }
    }

    async fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        Ok(rocket.manage(Store::init()))
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data) {
        let store = req.managed_state::<Store>().unwrap();

        if rand::thread_rng().gen::<f64>() <= self.config.lottery() {
            store.tidy().await.unwrap();
        }

        req.local_cache_async(async {
            self.init_session(req.cookies().get("session_id"), &store, &self.config)
                .await
        })
        .await;
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let store = req.managed_state::<Store>().unwrap();
        let session: &Session<Store::SessionData> = req.local_cache(Session::default);

        store.store(session.clone()).await.unwrap();

        let (session_cookie_name, session_value) = session.cookie_value();
        if !cookie_value_exists(req.cookies(), session_cookie_name, session_value.as_str()) {
            res.adjoin_header(make_cookie(
                &self.config,
                session_cookie_name,
                session_value.as_str(),
            ));
        }
    }
}
