use rand::{distributions::Alphanumeric, Rng};
use rocket::http::CookieJar;

pub(crate) fn private_cookie_exists(cookie_jar: &CookieJar<'_>, cookie_name: &str) -> bool {
    cookie_jar.get_private(cookie_name).is_some()
        || cookie_jar.get_private_pending(cookie_name).is_some()
}

pub(crate) fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(40)
        .map(|c| c as char)
        .collect()
}
