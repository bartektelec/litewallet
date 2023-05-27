use diesel::prelude::*;
use rocket::*;

mod common;
mod routes;
mod schema;
mod services;
mod utils;

use common::{db, models};
use routes::accounts;
use routes::auth;

#[get("/")]
fn index() -> String {
    use crate::schema::users::dsl::*;

    let connection = &mut db::establish_connection();

    let results = users
        .load::<models::User>(connection)
        .expect("error loading user");

    format!("Users: {:?}", results)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/account",
            routes![
                accounts::post_account,
                accounts::get_accounts,
                accounts::post_transfer
            ],
        )
        .mount(
            "/auth",
            routes![
                auth::get_me,
                auth::get_signout,
                auth::post_signin,
                auth::post_signup
            ],
        )
}
