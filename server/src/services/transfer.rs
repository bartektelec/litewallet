use diesel::prelude::*;
use diesel::result;

use crate::common::{db, models};
use crate::schema::accounts;
use crate::services::account;
use crate::utils::generate_acc_num::generate_acc_num;

pub fn transfer(from_acc_id: String, to_acc_id: String, amount: String) -> Result<(), String> {
    let to_acc = account::get_account_by_acc_id(&to_acc_id)
        .map_err(|_| "This account doesnt exist".to_string())?;

    // does from acc have enough balance
    //
    // does to_acc exist?
    //
    // is both from and to active

    // let payload = models::NewAccount {
    //     owner_id: user_id,
    //     account_number: &acc_num,
    // };
    //
    // let conn = &mut db::establish_connection();
    //
    // diesel::insert_into(accounts::table)
    //     .values(&payload)
    //     .execute(conn)?;
    //
    // get_account_by_acc_id(&acc_num)
    //
    Ok(())
}
