use super::DatabaseResult;
use crate::authentication::gaurd;
use crate::db::DbConn;
use crate::models::{Account, NewAccount};
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AccountData {
    pub name: String,
    pub user_id: String,
}

// Admin User has no control over other user's accounts

/// get all account
#[get("/accounts")]
pub fn get_all_accounts(user: gaurd::UserGaurd, mut conn: DbConn) -> Option<Json<Vec<Account>>> {
    if let DatabaseResult::Succeful(acc_vec) = Account::all(&mut conn, user.username) {
        Some(Json(acc_vec))
    } else {
        None
    }
}

/// get an account with id
#[get("/accounts/<identifier>")]
pub fn get_account(
    identifier: i32,
    user: gaurd::UserGaurd,
    mut conn: DbConn,
) -> Option<Json<Account>> {
    if let DatabaseResult::Succeful(acc) = Account::get(&mut conn, identifier) {
        Some(Json(acc))
    } else {
        None
    }
}

/// create an account
#[post("/accounts", format = "application/json", data = "<new_account>")]
pub fn create_account(
    new_account: Json<AccountData>,
    user: gaurd::UserGaurd,
    mut conn: DbConn,
) -> Option<Json<Account>> {
    let new_account = new_account.0;
    if let DatabaseResult::Succeful(acc) = Account::add(&mut conn, &new_account.into()) {
        Some(Json(acc))
    } else {
        None
    }
}

// update an account
#[patch(
    "/accounts/<identifier>",
    format = "application/json",
    data = "<account>"
)]
pub fn update_account(
    identifier: i32,
    account: Json<Account>,
    user: gaurd::UserGaurd,
    mut conn: DbConn,
) -> Option<Json<Account>> {
    let new_update = account.0;
    if let DatabaseResult::Succeful(acc) = Account::update(&mut conn, identifier, &new_update) {
        Some(Json(acc))
    } else {
        None
    }
}

// delete an account
#[delete("/accounts/<identifier>")]
pub fn delete_account(
    identifier: i32,
    user: gaurd::UserGaurd,
    mut conn: DbConn,
) -> Option<Json<Account>> {
    if let DatabaseResult::Succeful(acc) = Account::delete_by_id(&mut conn, identifier) {
        Some(Json(acc))
    } else {
        None
    }
}
