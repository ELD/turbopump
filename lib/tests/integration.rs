use rocket::{get, http::Status, local::blocking::Client, response::content::Html, routes, Rocket};
use turbopump::{
    fairing::config::SessionConfig, fairing::SessionFairing, store::in_memory::InMemory, Session,
};

#[derive(Clone, Default)]
struct HitCounter {
    count: u32,
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(SessionFairing::<InMemory<HitCounter>>::with_config(
            session_config(),
        ))
        .mount("/", routes![test_route])
}

fn session_config() -> SessionConfig {
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

#[test]
fn it_sets_a_session_cookie() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let result = client.get("/").dispatch();

    assert_eq!(result.status(), Status::Ok);
    assert!(result.cookies().get_private("session_id").is_some());
}
