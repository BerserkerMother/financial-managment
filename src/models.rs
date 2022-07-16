use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::schema::sql_types::CurrencyType)]
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
