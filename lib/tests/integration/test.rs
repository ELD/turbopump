use rocket::{http::Status, local::blocking::Client};

use crate::{
    app::{rocket, session_config},
    helpers::make_cookie,
};

#[test]
fn it_sets_a_session_cookie() {
    let client = Client::tracked(rocket()).expect("could not instantiate rocket");
    let result = client.get("/").dispatch();

    assert_eq!(result.status(), Status::Ok);
    assert!(result.cookies().get("session_id").is_some());
}

#[test]
fn it_rejects_an_invalid_session_cookie() {
    let client = Client::tracked(rocket()).expect("could not instantiate rocket");
    let (fixture_name, fixture_value) = ("session_id", "garbage");
    let result = client
        .get("/")
        .cookie(make_cookie(&session_config(), fixture_name, fixture_value))
        .dispatch();

    assert_eq!(result.status(), Status::Ok);
    assert!(result.headers().contains("Set-Cookie"));
    assert!(result.cookies().get(fixture_name).unwrap().value() != fixture_value);
    assert!(result.headers().get("Set-Cookie").next().unwrap() != fixture_value);
    assert!(!result
        .headers()
        .get("Set-Cookie")
        .next()
        .unwrap()
        .is_empty());
}
