use diesel::prelude::*;

use crate::common::db::establish_connection;
use crate::services::account;

pub fn transfer(from_acc_id: &String, to_acc_id: &String, amount: &String) -> Result<(), String> {
    let to_acc = account::get_account_by_acc_id(to_acc_id)
        .map_err(|_| "This target account doesnt exist".to_string())?;

    // does from acc have enough balance

    let from_acc = account::get_account_by_acc_id(from_acc_id)
        .map_err(|_| "This origin account doesnt exist".to_string())?;

    let amount_num = amount.parse::<u64>().map_err(|e| e.to_string())?;
    let from_acc_balance = from_acc.balance.parse::<u64>().map_err(|e| e.to_string())?;
    let to_acc_balance = to_acc.balance.parse::<u64>().map_err(|e| e.to_string())?;

    if amount_num > from_acc_balance {
        return Err("Not enough balance".to_string());
    }

    if to_acc.active == 0 {
        return Err("Target account is not active".to_string());
    }

    if from_acc.active == 0 {
        return Err("Origin account is not active".to_string());
    }

    let after_from_balance = from_acc_balance - amount_num;
    let after_to_balance = to_acc_balance + amount_num;

    let connection = &mut establish_connection();

    use crate::schema::accounts::dsl::*;
    connection
        .transaction(|conn| {
            diesel::update(accounts)
                .filter(account_number.eq(from_acc_id))
                .set(balance.eq(after_from_balance.to_string()))
                .execute(conn)?;
            diesel::update(accounts)
                .filter(account_number.eq(to_acc_id))
                .set(balance.eq(after_to_balance.to_string()))
                .execute(conn)?;

            diesel::result::QueryResult::Ok(())
        })
        .map_err(|_| "Updating account balance failed".to_string())?;

    Ok(())
}
