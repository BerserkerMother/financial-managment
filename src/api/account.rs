use super::establish_connection;
use super::DatabaseResult;
use crate::authentication::gaurd;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AccountData {
    pub name: String,
    pub username: String,
}

// Admin User has no control over other user's accounts

/// get all account
#[get("/accounts")]
pub fn get_all_accounts(user: gaurd::UserGaurd) {}

/// get an account with id
#[get("/accounts/<identifier>")]
pub fn get_account(identifier: i32, user: gaurd::UserGaurd) {}

/// create an account
#[post("/accounts", format = "application/json", data = "<new_account>")]
pub fn create_account(new_account: Json<AccountData>) {}

// update an account
#[patch(
    "/accounts/<identifier>",
    format = "application/json",
    data = "<account>"
)]
pub fn update_account(identifier: i32, account: Json<AccountData>, user: gaurd::UserGaurd) {}

// delete an account
#[delete("/accounts/<identifier>")]
pub fn remove_account(identifier: i32, user: gaurd::UserGaurd) {}
