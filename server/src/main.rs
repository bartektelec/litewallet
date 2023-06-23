mod common;
mod routes;
mod schema;
mod services;
mod utils;

use diesel::prelude::*;
use rocket::http::{ContentType, Header, Method, Status};
use rocket::*;

use common::{db, models};
use routes::accounts;
use routes::auth;

use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if request.method() == Method::Options {
            let body = "";
            response.set_header(ContentType::Plain);
            response.set_sized_body(body.len(), std::io::Cursor::new(body));
            response.set_status(Status::Ok);
        }
    }
}

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
        .attach(CORS)
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
