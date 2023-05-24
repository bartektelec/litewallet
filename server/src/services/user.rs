use crate::common::{db, models};
use crate::schema::users;
use diesel::prelude::*;
use diesel::result;

pub fn get_all() -> Result<Vec<models::User>, result::Error> {
    use crate::schema::users::dsl::*;

    let conn = &mut db::establish_connection();

    users.load(conn)
}

pub fn get_by_name(name: &str) -> Result<models::User, result::Error> {
    use crate::schema::users::dsl::*;

    let conn = &mut db::establish_connection();

    let results: Vec<models::User> = users.filter(username.eq(name)).load(conn)?;

    match results.into_iter().nth(0) {
        Some(v) => Ok(v),
        None => Err(result::Error::NotFound),
    }
}

pub fn create<T: AsRef<str>, U: AsRef<str>>(
    name: T,
    pass: U,
) -> Result<models::User, result::Error> {
    let payload = models::NewUser {
        username: name.as_ref(),
        pass: pass.as_ref(),
    };

    let conn = &mut db::establish_connection();

    let found = get_by_name(name.as_ref());

    if let Ok(_) = found {
        return Err(result::Error::NotFound);
    }

    diesel::insert_into(users::table)
        .values(&payload)
        .execute(conn)?;

    get_by_name(name.as_ref())
}
// #[get("/")]
// fn index() -> String {
//     use crate::schema::users::dsl::*;
//
//     let connection = &mut db::establish_connection();
//
//     let new_user = models::NewUser {
//         username: "bartek",
//         pass: "12345",
//     };
//
//     diesel::insert_into(crate::schema::users::table)
//         .values(&new_user)
//         .execute(connection)
//         .expect("error saving");
//
//     let results = users
//         .filter(id.gt(1))
//         .load::<models::User>(connection)
//         .expect("error loading user");
//
//     format!("Users: {:?}", results)
// }
