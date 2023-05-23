use diesel::prelude::*;
use rocket::*;

mod common;
mod routes;
mod schema;

use crate::common::{db, models};
use crate::schema::users;

#[get("/")]
fn index() -> String {
    use crate::schema::users::dsl::*;

    let connection = &mut db::establish_connection();

    let new_user = models::NewUser {
        id: 0,
        username: "bartek",
        pass: "12345",
    };

    diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("error saving");

    let results = users
        .load::<models::User>(connection)
        .expect("error loading user");

    format!("Users: {:?}", results)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount(
        "/auth",
        routes![routes::auth::get_me, routes::auth::post_signin],
    )
}
