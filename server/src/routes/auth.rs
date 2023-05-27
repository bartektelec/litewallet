use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::*;

use crate::services::user;
use rocket::response::status::{BadRequest, NotFound};
use sha256::digest;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetMeResponse {
    id: i32,
    username: String,
}

#[get("/me")]
pub async fn get_me(jar: &CookieJar<'_>) -> Result<Json<GetMeResponse>, NotFound<()>> {
    let opt_name = jar.get("sid");
    let sid = opt_name.ok_or(NotFound(()))?;
    let session_id = sid.value();

    let result = user::retrieve_user_from_sid(session_id);

    let me = result.map_err(|_| NotFound(()))?;

    let response = GetMeResponse {
        id: me.id,
        username: me.username,
    };

    Ok(response.into())
}
#[get("/signout")]
pub fn get_signout(jar: &CookieJar<'_>) -> Option<()> {
    jar.get("sid")?;

    jar.remove(Cookie::named("sid"));

    Some(())
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
) -> Result<Json<GetMeResponse>, BadRequest<()>> {
    let found = user::get_by_name(creds.username).map_err(|_| BadRequest(None))?;

    let hashed_pass = digest(creds.pass);

    if hashed_pass != found.pass {
        return Err(BadRequest(None));
    }

    let session = user::generate_session(found.id).map_err(|_| BadRequest(None))?;

    cookies.add(Cookie::new("sid", session));

    let me = user::get_by_name(creds.username).map_err(|_| BadRequest(None))?;

    let response = GetMeResponse {
        id: me.id,
        username: me.username,
    };

    Ok(response.into())
}

#[post("/signup", data = "<creds>")]
pub async fn post_signup(
    creds: Json<Credentials<'_>>,
) -> Result<Json<GetMeResponse>, BadRequest<String>> {
    let hashed_pass = digest(creds.pass);

    let user_result =
        user::create(creds.username, hashed_pass).map_err(|e| BadRequest(Some(e.to_string())))?;

    user::create_session(user_result.id).map_err(|e| BadRequest(Some(e.to_string())))?;

    let me = user::get_by_name(creds.username).map_err(|_| BadRequest(None))?;

    let response = GetMeResponse {
        id: me.id,
        username: me.username,
    };

    Ok(response.into())
}
