use rocket::http::SameSite as RSameSite;
use serde::{
    de::{self, Deserialize as DeserializeTrait, Visitor},
    Deserialize,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Into<RSameSite> for SameSite {
    fn into(self) -> RSameSite {
        match self {
            Self::Strict => RSameSite::Strict,
            Self::Lax => RSameSite::Lax,
            Self::None => RSameSite::None,
        }
    }
}

impl<'de> DeserializeTrait<'de> for SameSite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SameSiteVisitor;

        impl<'de> Visitor<'de> for SameSiteVisitor {
            type Value = SameSite;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("`Strict`, `Lax`, or `None`")
            }

            fn visit_str<E>(self, value: &str) -> Result<SameSite, E>
            where
                E: de::Error,
            {
                match value.to_lowercase().as_ref() {
                    "strict" => Ok(SameSite::Strict),
                    "lax" => Ok(SameSite::Lax),
                    "none" => Ok(SameSite::None),
                    _ => Err(de::Error::unknown_field(value, &["strict", "lax", "none"])),
                }
            }
        }

        deserializer.deserialize_str(SameSiteVisitor)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SessionConfig {
    pub max_age: i32,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub same_site: SameSite,
    pub http_only: bool,
}

impl SessionConfig {}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_age: 3600,
            domain: None,
            path: Some("/".to_string()),
            same_site: SameSite::None,
            http_only: false,
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
        "#;

        let expected_config = SessionConfig {
            max_age: 3600,
            domain: Some("example.local".to_string()),
            path: Some("/".to_string()),
            same_site: SameSite::Lax,
            http_only: true,
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
