//! # Turbopump - API Documentation
//!
//! Turbopump is a session middleware for the [Rocket](https://rocket.rs) web
//! framework. The building blocks of the session middleware are in the
//! [session] module and can theoretically be extracted for use with other
//! web frameworks.
//!
//! Most often, you'll find other libraries consuming this one, rather than
//! using it directly. However, in case you are using it directly for session
//! storage and persistence, these are the docs that will go over the types,
//! traits, and various usages of them.
//!
//! ## Usage
//!
//! Depend on `turbopump` in `Cargo.toml` in addition to [Rocket](https://rocket.rs).
//! ```toml
//! turbopump = "0.1.0"
//! ```
//!
//! Begin using it in your application:
//! ```rust
//! use rocket::{error::Error as RocketError, get, response::content::Html, routes};
//! use turbopump::{
//!     fairing::{config::SessionConfig, SessionFairing},
//!     store::in_memory::InMemory,
//!     Session,
//! };
//!
//! #[derive(Clone, Default)]
//! pub struct HitCounter {
//!     count: u32,
//! }
//!
//! #[rocket::launch]
//! fn rocket() -> _ {
//!     rocket::ignite()
//!         .attach(SessionFairing::<InMemory<HitCounter>>::with_config(
//!             SessionConfig::builder().finish(),
//!         ))
//!         .mount("/", routes![hit_counter])
//! }
//!
//! #[get("/hit-counter")]
//! async fn hit_counter(session: &Session<HitCounter>) -> Html<String> {
//!     let count = session.tap(|counter| {
//!         counter.count += 1;
//!         counter.count
//!     });
//!
//!     Html(format!(
//!         r##"
//!             <h1>Hello, world!</h1>
//!             <p>You've visited this page {} times</p>
//!         "##,
//!         count
//!     ))
//! }
//! ```
//!
//! ## Features
//!
//! There are currently no features to be turned on or off. With time, and
//! more session store types, there will be features to turn support for them
//! on or off.
//!
//! ## Configuration
//!
//! Session duration, cookie settings, and more can be configured separately or
//! via the `Rocket.toml` file in your Rocket application.
//!
//! For more details, check out the [`fairing::config::SessionConfig`] documentation.

pub mod fairing;
pub mod session;
pub mod store;
pub mod types;
mod util;

#[doc(inline)]
pub use crate::session::Session;
#[doc(inline)]
pub use crate::store::in_memory::InMemory;
#[doc(inline)]
pub use crate::store::SessionStore;
#[doc(inline)]
pub use crate::types::SessionId;
