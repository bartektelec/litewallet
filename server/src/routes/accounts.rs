use rocket::http::{Cookie, CookieJar};
use rocket::serde::{json::Json, Deserialize};
use rocket::*;

use crate::common::{db, models};
use crate::services;
use rocket::response::status::{BadRequest, NotFound};

#[post("/create")]
pub async fn post_account(jar: &CookieJar<'_>) -> Option<String> {
    let opt_name = jar.get("sid")?;
    let session_id = opt_name.value();

    let user = services::user::retrieve_user_from_sid(session_id).ok()?;

    let account = services::account::create_account(user.id).ok()?;

    Some(account.account_number)
}

#[post("/get")]
pub async fn get_accounts(jar: &CookieJar<'_>) -> Option<String> {
    let opt_name = jar.get("sid")?;
    let session_id = opt_name.value();

    let user = services::user::retrieve_user_from_sid(session_id).ok()?;

    let accounts = services::account::get_acc_list(user.id);

    Some(format!("{:?}", accounts))
}
