use super::establish_connection;
use super::DatabaseResult;
use crate::authentication::gaurd;
use crate::db::DbConn;
use crate::models::{CurrencyType, NewTransaction, Transaction};
use diesel::PgConnection;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TransactionData {
    pub kind: bool,
    pub title: String,
    pub value: String,
    pub currency: CurrencyType,
    pub user_id: String,
    pub bank_account: i32,
}

// admin has no control on user data

/// Post to create a new transaction
#[post(
    "/transaction",
    format = "application/json",
    data = "<new_transaction>"
)]
pub fn create_transaction(
    new_transaction: Json<TransactionData>,
    user: gaurd::UserGaurd,
    mut conn: DbConn,
) -> Option<Json<Transaction>> {
    let trans = new_transaction.0;
    if let DatabaseResult::Succeful(trans) = Transaction::add(&mut conn, &trans.into()) {
        Some(Json(trans))
    } else {
        None
    }
}

/// Get to retrieve a transaction
#[get("/transaction/<identifier>")]
pub fn get_transaction(
    identifier: i32,
    user: gaurd::UserGaurd,
    mut conn: DbConn,
) -> Option<Json<Transaction>> {
    if let DatabaseResult::Succeful(trans) = Transaction::get(&mut conn, identifier) {
        Some(Json(trans))
    } else {
        None
    }
}

/// Get to retrieve an account's all transactions
#[get("/transaction?<account_id>")]
pub fn get_account_all_transactions(
    account_id: i32,
    user: gaurd::UserGaurd,
    mut conn: DbConn,
) -> Option<Json<Vec<Transaction>>> {
    if let DatabaseResult::Succeful(trans_vec) = Transaction::all(&mut conn, account_id) {
        Some(Json(trans_vec))
    } else {
        None
    }
}

/// Delete to remove a transaction
#[delete("/transaction/<identifier>")]
pub fn delete_transaction(
    identifier: i32,
    user: gaurd::UserGaurd,
    mut conn: DbConn,
) -> Option<Json<Transaction>> {
    if let DatabaseResult::Succeful(trans) = Transaction::delete(&mut conn, identifier) {
        Some(Json(trans))
    } else {
        None
    }
}

/// Delete to remove an account's all transactions
#[delete("/transaction?<account_id>")]
pub fn delete_account_all_transactions(
    account_id: i32,
    user: gaurd::UserGaurd,
    mut conn: DbConn,
) -> Option<Json<Vec<Transaction>>> {
    if let DatabaseResult::Succeful(trans_vec) = Transaction::delete_all(&mut conn, account_id) {
        Some(Json(trans_vec))
    } else {
        None
    }
}
