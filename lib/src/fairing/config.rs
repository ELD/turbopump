use std::borrow::Cow;

use rocket::http::SameSite;
use serde::{de::Error, Deserialize, Deserializer};

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
    pub fn builder() -> SessionConfigBuilder {
        SessionConfigBuilder::default()
    }

    pub fn max_age(&self) -> i64 {
        self.max_age
    }

    pub fn domain(&self) -> Option<&str> {
        self.domain.as_ref().map(|inner| inner.as_ref())
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_ref().map(|inner| inner.as_ref())
    }

    pub fn same_site(&self) -> SameSite {
        self.same_site
    }

    pub fn http_only(&self) -> bool {
        self.http_only
    }

    pub fn lottery(&self) -> f64 {
        self.lottery
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfigBuilder::default().finish()
    }
}

pub struct SessionConfigBuilder {
    max_age: i64,
    domain: Option<&'static str>,
    path: Option<&'static str>,
    same_site: SameSite,
    http_only: bool,
    lottery: f64,
}

impl SessionConfigBuilder {
    pub fn max_age(self, max_age: i64) -> Self {
        Self { max_age, ..self }
    }

    pub fn domain(self, domain: &'static str) -> Self {
        Self {
            domain: Some(domain),
            ..self
        }
    }

    pub fn path(self, path: &'static str) -> Self {
        Self {
            path: Some(path),
            ..self
        }
    }

    pub fn same_site(self, same_site: SameSite) -> Self {
        Self { same_site, ..self }
    }

    pub fn http_only(self, http_only: bool) -> Self {
        Self { http_only, ..self }
    }

    pub fn lottery(self, lottery: f64) -> Self {
        Self { lottery, ..self }
    }

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
