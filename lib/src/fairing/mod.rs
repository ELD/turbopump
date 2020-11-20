use std::marker::PhantomData;

use async_trait::async_trait;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Cookie,
    Data, Request, Response, Rocket,
};

use crate::{
    fairing::config::SessionConfig, store::SessionStore, util::private_cookie_exists, Session,
};

pub mod config;

pub struct SessionFairing<Store: SessionStore> {
    config: Option<SessionConfig>,
    store: PhantomData<Store>,
}

impl<Store: SessionStore> SessionFairing<Store> {
    pub fn init() -> Self {
        Self {
            store: PhantomData,
            config: None,
        }
    }

    pub fn with_config(config: SessionConfig) -> Self {
        Self {
            store: PhantomData,
            config: Some(config),
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
        let config = if let Some(config) = self.config.clone() {
            config
        } else {
            rocket
                .figment()
                .extract_inner::<SessionConfig>("session")
                .expect("unable to extract session config")
        };
        // Store the SessionStore in managed state
        Ok(rocket
            .manage(Box::new(Store::init()) as Box<Store>)
            .manage(config))
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data) {
        req.local_cache_async(async {
            let store = req.managed_state::<Box<Store>>().unwrap();
            let session = if let Some(session_cookie) = req.cookies().get_private("session_id") {
                store
                    .load(session_cookie.value().into())
                    .await
                    .unwrap()
                    .unwrap_or_else(Session::new)
            } else {
                Session::new()
            };

            let session_cookie = session.cookie_value();
            let xsrf_cookie = session.token_value();
            let jar = req.cookies();
            // ensure the cookie exists
            if !private_cookie_exists(jar, session_cookie.0) {
                jar.add_private(Cookie::new(
                    session_cookie.0.to_string(),
                    session_cookie.1.to_string(),
                ));
            }

            if !private_cookie_exists(jar, xsrf_cookie.0) {
                jar.add_private(Cookie::new(
                    xsrf_cookie.0.to_string(),
                    xsrf_cookie.1.to_string(),
                ));
            }

            session
        })
        .await;
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, _res: &mut Response<'r>) {
        // Store the session before finalizing the response
        let session: &Session<Store::SessionData> = req.local_cache(Session::new);
        let store = req.managed_state::<Box<Store>>().unwrap();
        store.store(session.clone()).await.unwrap();
    }
}
