use super::schema::transaction;
use super::*;
use serde::Deserialize;

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

#[derive(Debug, Insertable)]
#[table_name = "transaction"]
pub struct NewTransaction<'a> {
    pub kind: bool,
    pub title: Option<&'a str>,
    pub value: &'a str,
    pub currency: Option<CurrencyType>,
    pub time: NaiveDate,
    pub user_id: &'a str,
    pub bank_account: i32,
}

impl<'a> NewTransaction<'a> {
    fn new(
        kind: bool,
        title: Option<&'a str>,
        value: &'a str,
        currency: Option<CurrencyType>,
        time: (i32, u32, u32),
        user_id: &'a str,
        bank_account: i32,
    ) -> NewTransaction<'a> {
        let time = chrono::NaiveDate::from_ymd(time.0, time.1, time.2);

        NewTransaction {
            kind,
            title,
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
            title: Some("Huh"),
            value: "344134000",
            currency: Some(CurrencyType::USD),
            time: chrono::NaiveDate::from_ymd(2000, 1, 1),
            user_id: "BerserkerMother",
            bank_account: 1,
        }
    }
}

#[derive(Debug, AsExpression, FromSqlRow, PartialEq, Deserialize)]
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
// make sure a test user with username "BerserkerMother" exist in database
