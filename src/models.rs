use chrono::NaiveDate;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::cmp::PartialEq;
use std::io::Write;

use super::schema::{account, transaction, users};

#[derive(Queryable, Debug, PartialEq)]
pub struct Account {
    pub balance: Option<String>,
    pub user_id: String,
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount<'a> {
    pub balance: Option<&'a str>,
    pub user_id: &'a str,
    pub name: &'a str,
}

impl<'a> NewAccount<'a> {
    pub fn new(user_id: &'a str, name: &'a str) -> NewAccount<'a> {
        NewAccount {
            balance: Some("0"),
            user_id,
            name,
        }
    }
}

impl<'a> Default for NewAccount<'a> {
    fn default() -> NewAccount<'a> {
        NewAccount {
            balance: Some("0"),
            user_id: "BerserkerMother",
            name: "American Express",
        }
    }
}

#[derive(Queryable, Debug, PartialEq)]
pub struct Transaction {
    pub kind: bool,
    pub source: Option<String>,
    pub note: Option<String>,
    pub value: String,
    pub currency: Option<CurrencyType>,
    pub time: NaiveDate,
    pub user_id: String,
    pub id: i32,
    pub bank_account: Option<i32>,
}

#[derive(Queryable, Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn new(name: &'a str, username: &'a str, password: &'a str) -> NewUser<'a> {
        NewUser {
            name,
            username,
            password,
        }
    }
}

impl<'a> Default for NewUser<'a> {
    fn default() -> NewUser<'a> {
        NewUser {
            name: "Kimia",
            username: "absolute_trash",
            password: "huh",
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = transaction)]
pub struct NewTransaction<'a> {
    pub kind: bool,
    pub source: Option<&'a str>,
    pub note: Option<&'a str>,
    pub value: &'a str,
    pub currency: Option<CurrencyType>,
    pub time: NaiveDate,
    pub user_id: &'a str,
    pub bank_account: Option<i32>,
}

impl<'a> NewTransaction<'a> {
    pub fn new(
        kind: bool,
        source: Option<&'a str>,
        note: Option<&'a str>,
        value: &'a str,
        currency: Option<CurrencyType>,
        time: (i32, u32, u32),
        user_id: &'a str,
        bank_account: Option<i32>,
    ) -> NewTransaction<'a> {
        let time = chrono::NaiveDate::from_ymd(time.0, time.1, time.2);

        NewTransaction {
            kind,
            source,
            note,
            value,
            currency,
            time,
            user_id,
            bank_account,
        }
    }
}

impl<'a> Default for NewTransaction<'a> {
    fn default() -> NewTransaction<'a> {
        NewTransaction {
            kind: true,
            source: Some("Huh"),
            note: Some("aksdj kaskjd"),
            value: "344134000",
            currency: Some(CurrencyType::USD),
            time: chrono::NaiveDate::from_ymd(2000, 01, 01),
            user_id: "BerserkerMother",
            bank_account: None,
        }
    }
}

#[derive(Debug, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = crate::schema::sql_types::CurrencyType)]
/// Enum representing currency_type for postgres database
pub enum CurrencyType {
    USD,
    IRR,
    CAD,
    Euruo,
}

impl ToSql<crate::schema::sql_types::CurrencyType, Pg> for CurrencyType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            CurrencyType::USD => out.write_all(b"USD")?,
            CurrencyType::IRR => out.write_all(b"IRR")?,
            CurrencyType::CAD => out.write_all(b"CAD")?,
            CurrencyType::Euruo => out.write_all(b"Euruo")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::CurrencyType, Pg> for CurrencyType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"USD" => Ok(CurrencyType::USD),
            b"IRR" => Ok(CurrencyType::IRR),
            b"CAD" => Ok(CurrencyType::CAD),
            b"Euruo" => Ok(CurrencyType::Euruo),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
