use rocket::{error::Error as RocketError, get, response::content::Html, routes};
use turbopump::{
    fairing::{SessionConfig, SessionFairing},
    InMemory, Session,
};

#[derive(Clone, Default)]
pub struct HitCounter {
    count: u32,
}

#[rocket::main]
async fn main() -> Result<(), RocketError> {
    rocket::ignite()
        .attach(SessionFairing::<InMemory<HitCounter>>::with_config(
            SessionConfig::builder().finish(),
        ))
        .mount("/", routes![hit_counter, bare_route])
        .launch()
        .await
}

#[get("/empty-route")]
async fn bare_route() -> Html<String> {
    Html("<h1>Just a bare route!</h1>".to_string())
}

#[get("/hit-counter")]
async fn hit_counter(session: &Session<HitCounter>) -> Html<String> {
    let count = session.tap(|counter| {
        counter.count += 1;
        counter.count
    });

    Html(format!(
        r##"
    <h1>Hello, world!</h1>
    <p>You've visited this page {} times</p>"##,
        count
    ))
}
