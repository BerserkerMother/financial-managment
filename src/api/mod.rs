pub mod account;
pub mod transaction;
pub mod user;

use crate::establish_connection;
use crate::models::result_variant::DatabaseResult;

use rocket::Route;
use user::create_user;
pub fn stage() -> Vec<Route> {
    routes![create_user]
}
