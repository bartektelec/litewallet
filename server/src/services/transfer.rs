use diesel::prelude::*;
use diesel::result;

use crate::common::{db, models};
use crate::schema::accounts;
use crate::services::account;
use crate::utils::generate_acc_num::generate_acc_num;

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

    Ok(())
}
