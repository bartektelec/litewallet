use rocket::data::FromData;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::{json::Json, Deserialize};
use rocket::*;

#[get("/me")]
pub async fn get_me(jar: &CookieJar<'_>) -> String {
    let opt_name = jar.get("name");

    if let Some(v) = opt_name {
        let name = v.value();

        return format!("Hello you {}", name);
    } else {
        return "Not logged in".into();
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials<'t> {
    username: &'t str,
    pass: &'t str,
}

#[post("/signin", data = "<creds>")]
pub async fn post_signin(cookies: &CookieJar<'_>, creds: Json<Credentials<'_>>) -> &'static str {
    cookies.add(Cookie::new("name", "bartek"));

    "Logged in"
}
