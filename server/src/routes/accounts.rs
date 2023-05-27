use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::*;

use crate::common::models::Account;
use crate::common::{db, models};
use crate::services;
use rocket::response::status::{BadRequest, NotFound};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateResponse {
    account_number: String,
}

#[post("/create")]
pub async fn post_account(jar: &CookieJar<'_>) -> Option<Json<CreateResponse>> {
    let opt_name = jar.get("sid")?;
    let session_id = opt_name.value();

    let user = services::user::retrieve_user_from_sid(session_id).ok()?;

    let account = services::account::create_account(user.id).ok()?;

    Some(
        CreateResponse {
            account_number: account.account_number,
        }
        .into(),
    )
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetAllResponse {
    data: Vec<Account>,
}

#[get("/")]
pub async fn get_accounts(jar: &CookieJar<'_>) -> Option<Json<GetAllResponse>> {
    let opt_name = jar.get("sid")?;
    let session_id = opt_name.value();

    let user = services::user::retrieve_user_from_sid(session_id).ok()?;

    let accounts = services::account::get_acc_list(user.id);

    Some(GetAllResponse { data: accounts }.into())
}
