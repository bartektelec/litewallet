use crate::schema::sessions;
use crate::schema::users;

use diesel::prelude::*;

#[derive(Queryable, Identifiable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pass: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub pass: &'a str,
}

#[derive(Queryable, Identifiable, Debug)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub session_id: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub user_id: i32,
    pub session_id: Option<String>,
}
