//! Types for interoperating with [`Rocket`](https://rocket.rs), and
//! configuration.
//!
//! This module contains the fairing implementation to allow for instantiating,
//! deleting, and accessing [`Session`](crate::session::Session)s.
//!
//! ## Attaching
//!
//! You must attach the fairing by calling [`Rocket::attach()`](rocket::Rocket::attach) on the
//! application's [`Rocket`](rocket::Rocket) instance. This also requires supplying the
//! [`SessionStore`](crate::store::SessionStore) instance, as well. For example:
//!
//! ```rust
//! # use rocket::{error::Error as RocketError, get, response::content::Html, routes};
//! # use turbopump::{
//! #     fairing::{SessionConfig, SessionFairing},
//! #     InMemory,
//! #     Session,
//! # };
//! #
//! # #[derive(Clone, Default)]
//! # pub struct Data {
//! #     count: u32,
//! # }
//! #
//! rocket::build().attach(SessionFairing::<InMemory<Data>>::init());
//! ```
//!
//! ## Ordering
//! Because of order dependecy with fairings, it's advised that you attach
//! this fairing before others, as it fetches the session cookie from the
//! incoming request, attempts to retrieve an already-started session (or
//! starts one), and then inserts it into the request-local cache.

mod config;
mod fairings;

pub use config::{SessionConfig, SessionConfigBuilder};
pub use fairings::SessionFairing;
