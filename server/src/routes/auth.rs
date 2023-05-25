use rocket::http::{Cookie, CookieJar};
use rocket::serde::{json::Json, Deserialize};
use rocket::*;

use crate::services::user;
use rocket::response::status::{BadRequest, NotFound};
use sha256::digest;

#[get("/me")]
pub async fn get_me(jar: &CookieJar<'_>) -> Result<String, NotFound<()>> {
    let opt_name = jar.get("sid").ok_or(NotFound(None))?;
    let session_id = opt_name.value();
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials<'t> {
    username: &'t str,
    pass: &'t str,
}

#[post("/signin", data = "<creds>")]
pub fn post_signin(
    cookies: &CookieJar<'_>,
    creds: Json<Credentials<'_>>,
) -> Result<String, BadRequest<()>> {
    let found = user::get_by_name(creds.username).map_err(|_| BadRequest(None))?;

    println!("found user {:?}", found);
    let hashed_pass = digest(creds.pass);

    if hashed_pass != found.pass {
        return Err(BadRequest(None));
    }

    let session = user::generate_session(found.id).map_err(|_| BadRequest(None))?;

    cookies.add(Cookie::new("sid", session));

    Ok("Logged in".to_string())
}

#[post("/signup", data = "<creds>")]
pub async fn post_signup(creds: Json<Credentials<'_>>) -> Result<String, BadRequest<String>> {
    let hashed_pass = digest(creds.pass);

    let user_result =
        user::create(creds.username, hashed_pass).map_err(|e| BadRequest(Some(e.to_string())))?;

    user::create_session(user_result.id).map_err(|e| BadRequest(Some(e.to_string())))?;

    Ok("Ok".to_string())
}
