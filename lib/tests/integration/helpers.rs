use rocket::http::Cookie;
use time::Duration;
use turbopump::fairing::SessionConfig;

pub(crate) fn make_cookie(
    config: &SessionConfig,
    name: impl Into<String>,
    value: impl Into<String>,
) -> Cookie<'_> {
    let mut builder = Cookie::build(name.into(), value.into())
        .max_age(Duration::new(config.max_age(), 0))
        .http_only(config.http_only())
        .same_site(config.same_site());

    if let Some(domain) = &config.domain().to_owned() {
        builder = builder.domain(domain.to_owned());
    };

    if let Some(path) = &config.path() {
        builder = builder.path(path.to_owned());
    };

    builder.finish()
}
