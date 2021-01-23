use rand::{distributions::Alphanumeric, Rng};
use rocket::http::{Cookie, CookieJar};
use time::Duration;

use crate::fairing::config::SessionConfig;

pub(crate) fn cookie_value_exists(
    cookie_jar: &CookieJar<'_>,
    cookie_name: &str,
    cookie_value: &str,
) -> bool {
    if let Some(cookie) = cookie_jar.get_pending(cookie_name) {
        cookie.value() == cookie_value
    } else if let Some(cookie) = cookie_jar.get_private_pending(cookie_name) {
        cookie.value() == cookie_value
    } else {
        false
    }
}

pub(crate) fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(40)
        .map(|c| c as char)
        .collect()
}

pub(crate) fn make_cookie<'a, 'c>(
    config: &'a SessionConfig,
    name: impl Into<String>,
    value: impl Into<String>,
) -> Cookie<'c> {
    let mut builder = Cookie::build(name.into(), value.into())
        .max_age(Duration::new(config.max_age, 0))
        .http_only(config.http_only)
        .same_site(config.same_site.into());

    if let Some(domain) = &config.domain {
        builder = builder.domain(domain.to_owned());
    };

    if let Some(path) = &config.path {
        builder = builder.path(path.to_owned());
    };

    builder.finish()
}

#[cfg(test)]
mod test {
    use rocket::http::{private::cookie::Key, Cookie, SameSite};
    use time::Duration;

    use crate::{fairing::config::SameSite as SSameSite, SessionId};

    use super::*;

    #[test]
    fn it_generates_random_strings() {
        let random_one = random_string();
        let random_two = random_string();

        assert!(random_one != random_two);
    }

    #[test]
    fn it_checks_if_cookie_exists_and_is_set_to_expected_value() {
        let key = Key::generate();
        let cookie_jar = CookieJar::new(&key);

        assert!(!cookie_value_exists(&cookie_jar, "session", ""));
        assert!(!cookie_value_exists(&cookie_jar, "unencrypted_cookie", ""));

        cookie_jar.add_private(Cookie::new("session", "sample_id"));
        assert!(cookie_value_exists(&cookie_jar, "session", "sample_id"));

        cookie_jar.add(Cookie::new("unencrypted_cookie", "example"));
        assert!(cookie_value_exists(
            &cookie_jar,
            "unencrypted_cookie",
            "example"
        ));

        cookie_jar.add(Cookie::new("session_id", "invalid"));
        assert!(!cookie_value_exists(&cookie_jar, "session_id", "valid"));
    }

    #[test]
    fn it_creates_cookie_from_config() {
        let config = SessionConfig {
            max_age: 3600,
            domain: Some("example.local".into()),
            path: Some("/".into()),
            same_site: SSameSite::Lax,
            http_only: true,
            lottery: 0.1,
        };
        let name = "session_cookie";
        let session_id = SessionId(random_string());

        let expected = Cookie::build(name, session_id.to_string())
            .max_age(Duration::new(3600, 0))
            .domain("example.local")
            .path("/")
            .same_site(SameSite::Lax)
            .http_only(true)
            .finish();

        assert_eq!(expected, make_cookie(&config, name, session_id.as_str()));
    }
}
