use crate::schema::{accounts, actions, sessions, users};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pass: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub pass: &'a str,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub session_id: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub user_id: i32,
    pub session_id: Option<String>,
}

#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Account {
    pub id: i32,
    pub owner_id: i32,
    pub account_number: String,
    pub balance: String,
    pub active: i32,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'t> {
    pub owner_id: i32,
    pub account_number: &'t String,
}

#[derive(Queryable, Identifiable, Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Action {
    pub id: i32,
    pub created_at: String,
    pub from_acc: String,
    pub to_acc: String,
    pub amount: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = actions)]
pub struct NewAction<'t> {
    pub created_at: &'t str,
    pub from_acc: &'t str,
    pub to_acc: &'t str,
    pub amount: String,
}
