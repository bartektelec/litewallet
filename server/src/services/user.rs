use crate::common::{db, models};
use crate::schema::{sessions, users};
use diesel::prelude::*;
use diesel::result;
use nanoid::nanoid;

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

pub fn get_by_id(input_id: i32) -> Result<models::User, result::Error> {
    use crate::schema::users::dsl::*;

    let conn = &mut db::establish_connection();

    let results: Vec<models::User> = users.filter(id.eq(input_id)).load(conn)?;

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

pub fn create_session(user_id: i32) -> Result<models::Session, result::Error> {
    let payload = models::NewSession {
        user_id,
        session_id: None,
    };

    let conn = &mut db::establish_connection();

    diesel::insert_into(sessions::table)
        .values(&payload)
        .execute(conn)?;

    get_session_by_user_id(user_id)
}

pub fn generate_session(input_user_id: i32) -> Result<String, result::Error> {
    use crate::schema::sessions::dsl::*;

    let s_id: String = nanoid!();

    let conn = &mut db::establish_connection();

    diesel::update(crate::schema::sessions::table)
        .filter(user_id.eq(input_user_id))
        .set(session_id.eq(&s_id))
        .execute(conn)?;

    Ok(s_id)
}

pub fn retrieve_user_from_sid(s_id: String) -> Result<models::User, result::Error> {
    let found_session = get_session_by_sid(s_id)?;

    get_by_id(found_session.id)
}

pub fn get_session_by_sid(s_id: String) -> Result<models::Session, result::Error> {
    use crate::schema::sessions::dsl::*;
    let conn = &mut db::establish_connection();

    let results: Vec<models::Session> = sessions.filter(session_id.eq(s_id)).load(conn)?;

    match results.into_iter().nth(0) {
        Some(v) => Ok(v),
        None => Err(result::Error::NotFound),
    }
}

pub fn get_session_by_user_id(input_user_id: i32) -> Result<models::Session, result::Error> {
    use crate::schema::sessions::dsl::*;
    let conn = &mut db::establish_connection();

    let results: Vec<models::Session> = sessions.filter(user_id.eq(input_user_id)).load(conn)?;

    match results.into_iter().nth(0) {
        Some(v) => Ok(v),
        None => Err(result::Error::NotFound),
    }
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
