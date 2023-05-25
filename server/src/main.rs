use diesel::prelude::*;
use rocket::*;

mod common;
mod routes;
mod schema;
mod services;

use common::{db, models};
use routes::auth;
use schema::users;

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
    rocket::build().mount("/", routes![index]).mount(
        "/auth",
        routes![auth::get_me, auth::post_signin, auth::post_signup],
    )
}
