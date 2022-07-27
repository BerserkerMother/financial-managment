pub mod account;
pub mod transaction;
pub mod user;

use crate::establish_connection;
use crate::models::result_variant::DatabaseResult;

use account::*;
use rocket::Route;
use transaction::*;
use user::*;
pub fn stage() -> Vec<Route> {
    routes![
        create_user,
        get_user,
        update_user,
        delete_user,
        super_get_all_user,
        super_get_user,
        super_update_user,
        super_delete_user,
        create_transaction,
        get_account_all_transactions,
        get_transaction,
        delete_account_all_transactions,
        delete_transaction,
        get_account,
        get_all_accounts,
        create_account,
        delete_account,
        update_account
    ]
}
