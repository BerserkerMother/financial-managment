use super::establish_connection;
use super::DatabaseResult;
use crate::authentication::gaurd;
use crate::models::CurrencyType;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TransactionData {
    kind: bool,
    source: Option<String>,
    value: String,
    currency: Option<CurrencyType>,
    username: String,
    bank_account: i32,
}

// admin has no control on user data

/// Post to create a new transaction
#[post(
    "/transaction",
    format = "application/json",
    data = "<new_transaction>"
)]
pub fn create_transaction(new_transaction: Json<TransactionData>, user: gaurd::UserGaurd) {}

/// Get to retrieve a transaction
#[get("/transaction/<identifier>")]
pub fn get_transaction(identifier: i32, user: gaurd::UserGaurd) {}

/// Get to retrieve an account's all transactions
#[get("/transaction?<account_id>")]
pub fn get_account_all_transactions(account_id: i32, user: gaurd::UserGaurd) {}

/// Delete to remove a transaction
#[delete("/transaction/<identifier>")]
pub fn delete_transaction(identifier: i32, user: gaurd::UserGaurd) {}

/// Delete to remove an account's all transactions
#[delete("/transaction?<account_id>")]
pub fn delete_account_all_transactions(account_id: i32, user: gaurd::UserGaurd) {}
