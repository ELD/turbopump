use std::borrow::Cow;

use rocket::http::SameSite;
use serde::{de::Error, Deserialize, Deserializer};

/// Type that contains configuration options for Turbopump sessions
///
/// This object is used to store the configuration options for Turbopump
/// sessions.
///
/// Creating a new configuration option should be used by calling
/// the [`SessionConfig::builder`] method and setting configuration values
/// on the builder object.
///
/// This type implements [`serde::Deserialize`] and thus can be loaded from
/// [`Figment`](https://docs.rs/figment/0.10.3/figment/) using provided methods.
///
/// For example:
///
/// ```rust
/// # use figment::{Figment, providers::{Format, Toml}};
/// # use turbopump::fairing::SessionConfig;
/// #
/// # let input = r#"
/// # [session]
/// # max_age = 3600
/// # domain = "example.local"
/// # path = "/"
/// # same_site = "lax"
/// # http_only = true
/// # lottery = 0.1
/// # "#;
/// #
/// let figment = Figment::from(Toml::string(input));
///
/// let deserialize_result: Result<SessionConfig, figment::Error> =
///     figment.extract_inner("session");
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SessionConfig {
    pub(crate) max_age: i64,
    pub(crate) domain: Option<Cow<'static, str>>,
    pub(crate) path: Option<Cow<'static, str>>,
    #[serde(deserialize_with = "deserialize_samesite")]
    pub(crate) same_site: SameSite,
    pub(crate) http_only: bool,
    pub(crate) lottery: f64,
}

impl SessionConfig {
    /// Creates a new [`SessionConfigBuilder`] for constructing a [`SessionConfig`]
    pub fn builder() -> SessionConfigBuilder {
        SessionConfigBuilder::default()
    }

    /// Returns the configured maximum age of a session
    pub fn max_age(&self) -> i64 {
        self.max_age
    }

    /// Returns the configured domain for the session cookie
    pub fn domain(&self) -> Option<&str> {
        self.domain.as_ref().map(|inner| inner.as_ref())
    }

    /// Returns the configured path for the session cookie
    pub fn path(&self) -> Option<&str> {
        self.path.as_ref().map(|inner| inner.as_ref())
    }

    /// Returns the SameSite configuration for the session cookie
    pub fn same_site(&self) -> SameSite {
        self.same_site
    }

    /// Returns whether the session cookie is set to be `HttpOnly`
    pub fn http_only(&self) -> bool {
        self.http_only
    }

    /// Returns the configured lottery value, as a fractional number, for when
    /// the session store sweeps out expired sessions
    pub fn lottery(&self) -> f64 {
        self.lottery
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfigBuilder::default().finish()
    }
}

/// Type that enables building a configuration object for Turbopump sessions
///
/// This type can be used to manually build up a session configuration object.
///
/// ### Example
///
/// ```
/// # use rocket::http::SameSite;
/// # use turbopump::fairing::SessionConfig;
/// let builder = SessionConfig::builder()
///     .max_age(144000)
///     .domain("api.example.dev")
///     .path("/")
///     .same_site(SameSite::Lax)
///     .http_only(true)
///     .finish();
/// ```
pub struct SessionConfigBuilder {
    max_age: i64,
    domain: Option<&'static str>,
    path: Option<&'static str>,
    same_site: SameSite,
    http_only: bool,
    lottery: f64,
}

impl SessionConfigBuilder {
    /// Set the max_age for the session
    pub fn max_age(self, max_age: i64) -> Self {
        Self { max_age, ..self }
    }

    /// Set the domain for the session cookie
    pub fn domain(self, domain: &'static str) -> Self {
        Self {
            domain: Some(domain),
            ..self
        }
    }

    /// Set the path for the session cookie
    pub fn path(self, path: &'static str) -> Self {
        Self {
            path: Some(path),
            ..self
        }
    }

    /// Set the `SameSite` attribute on the session cookie
    pub fn same_site(self, same_site: SameSite) -> Self {
        Self { same_site, ..self }
    }

    /// Set the `HttpOnly` attribute on the session cookie
    pub fn http_only(self, http_only: bool) -> Self {
        Self { http_only, ..self }
    }

    /// Set the lottery value for how often the session store should clear out
    /// expired sessions.
    pub fn lottery(self, lottery: f64) -> Self {
        Self { lottery, ..self }
    }

    /// Finalize the builder object and return a [`SessionConfig`]
    pub fn finish(self) -> SessionConfig {
        SessionConfig {
            max_age: self.max_age,
            domain: self.domain.map(|inner| inner.into()),
            path: self.path.map(|inner| inner.into()),
            same_site: self.same_site,
            http_only: self.http_only,
            lottery: self.lottery,
        }
    }
}

impl Default for SessionConfigBuilder {
    fn default() -> Self {
        Self {
            max_age: 3600,
            domain: None,
            path: Some("/"),
            same_site: SameSite::Lax,
            http_only: true,
            lottery: 0.1,
        }
    }
}

pub(crate) fn deserialize_samesite<'de, D>(deserializer: D) -> Result<SameSite, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.to_lowercase().as_ref() {
        "lax" => Ok(SameSite::Lax),
        "strict" => Ok(SameSite::Strict),
        "none" => Ok(SameSite::None),
        _ => Err(Error::custom(
            "value was not one of: `lax`, `strict`, or `none`.",
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use figment::{
        providers::{Format, Toml},
        Figment,
    };

    #[test]
    fn it_deserializes_a_valid_config() {
        let input = r#"
            [session]
            max_age = 3600
            domain = "example.local"
            path = "/"
            same_site = "lax"
            http_only = true
            lottery = 0.1
        "#;

        let expected_config = SessionConfig {
            max_age: 3600,
            domain: Some("example.local".into()),
            path: Some("/".into()),
            same_site: SameSite::Lax,
            http_only: true,
            lottery: 0.1,
        };

        let figment = Figment::from(Toml::string(input));

        let deserialize_result: Result<SessionConfig, figment::Error> =
            figment.extract_inner("session");

        assert!(deserialize_result.is_ok());
        assert_eq!(deserialize_result.unwrap(), expected_config);
    }

    #[test]
    fn it_fails_on_invalid_config() {
        let input = r#"
            [session]
            max_age = 3600
            domain = "example.local"
            path = "/"
            same_site = "invalid"
            http_only = true
        "#;

        let figment = Figment::from(Toml::string(input));

        let deserialize_result: Result<SessionConfig, figment::Error> =
            figment.extract_inner("session");

        assert!(deserialize_result.is_err());
    }
}
