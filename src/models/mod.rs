mod account;
mod transaction;
mod user;

use super::establish_connection;
use super::schema;
use chrono::NaiveDate;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

pub use account::{Account, NewAccount};
pub use transaction::{CurrencyType, NewTransaction, Transaction};
pub use user::{NewUser, User};

pub mod result_variant {
    /// Enum to represent state of database calls
    pub enum DatabaseResult<T> {
        Succeful(T),
        AlreadyExists,
        NotFound,
    }

    impl<T> DatabaseResult<T> {
        pub fn unwrap(self) -> T {
            match self {
                DatabaseResult::Succeful(item) => item,
                DatabaseResult::AlreadyExists => {
                    panic!("calling unwrap on AlreadyExists variant!!")
                }
                DatabaseResult::NotFound => panic!("calling unwrap on NotFound variant!!"),
            }
        }
    }
}

use result_variant::DatabaseResult;
