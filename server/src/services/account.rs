use diesel::prelude::*;
use diesel::result;

use crate::common::{db, models};
use crate::schema::accounts;
use crate::utils::generate_acc_num::generate_acc_num;

pub fn get_account_by_acc_id(acc_id: &String) -> Result<models::Account, result::Error> {
    use crate::schema::accounts::dsl::*;

    let conn = &mut db::establish_connection();

    let results: Vec<models::Account> = accounts.filter(account_number.eq(acc_id)).load(conn)?;

    match results.into_iter().nth(0) {
        Some(v) => Ok(v),
        None => Err(result::Error::NotFound),
    }
}

pub fn gen_valid_acc_id() -> String {
    let mut output = String::new();

    loop {
        let acc_id = generate_acc_num().to_string();

        if let Err(_) = get_account_by_acc_id(&acc_id) {
            output = acc_id;
            break;
        }
    }
    output
}

pub fn get_acc_list(user_id: i32) -> Vec<models::Account> {
    use crate::schema::accounts::dsl::*;

    let conn = &mut db::establish_connection();

    let results: Vec<models::Account> = accounts
        .filter(owner_id.eq(user_id))
        .load(conn)
        .ok()
        .unwrap_or(vec![]);

    results
}

pub fn create_account(user_id: i32) -> Result<models::Account, result::Error> {
    let acc_num = gen_valid_acc_id();

    let payload = models::NewAccount {
        owner_id: user_id,
        account_number: &acc_num,
    };

    let conn = &mut db::establish_connection();

    diesel::insert_into(accounts::table)
        .values(&payload)
        .execute(conn)?;

    get_account_by_acc_id(&acc_num)
}
