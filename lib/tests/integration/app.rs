use rocket::{get, response::content::Html, routes, Build, Rocket};
use turbopump::{fairing::SessionConfig, fairing::SessionFairing, InMemory, Session};

#[derive(Clone, Default)]
struct HitCounter {
    count: u32,
}

pub(crate) fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(SessionFairing::<InMemory<HitCounter>>::with_config(
            session_config(),
        ))
        .mount("/", routes![test_route])
}

pub(crate) fn session_config() -> SessionConfig {
    SessionConfig::default()
}

#[get("/")]
fn test_route(s: &Session<HitCounter>) -> Html<String> {
    let count = s.tap(|counter| {
        counter.count += 1;
        counter.count
    });

    Html(format!(
        r#"<h1>You have visited this page {} times</h1>"#,
        count,
    ))
}
