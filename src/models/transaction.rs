use super::schema::transaction;
use super::*;
use crate::models::result_variant::DatabaseResult;
use chrono::offset::Local;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub kind: bool,
    pub title: Option<String>,
    pub value: String,
    pub currency: CurrencyType,
    pub time: NaiveDate,
    pub user_id: String,
    pub id: i32,
    pub bank_account: i32,
}

impl Transaction {
    fn new(
        kind: bool,
        title: Option<String>,
        value: String,
        currency: CurrencyType,
        time: NaiveDate,
        user_id: String,
        id: i32,
        bank_account: i32,
    ) -> Transaction {
        Transaction {
            kind,
            title,
            value,
            currency,
            time,
            user_id,
            id,
            bank_account,
        }
    }

    /// adds a new transaction
    pub fn add(conn: &mut PgConnection, trans: &NewTransaction) -> DatabaseResult<Transaction> {
        match diesel::insert_into(transaction::table)
            .values(trans)
            .get_result::<Transaction>(conn)
        {
            Ok(trans) => DatabaseResult::Succeful(trans),
            Err(Error::DatabaseError(_, _)) => DatabaseResult::AlreadyExists,
            Err(err) => panic!("Something went wrong, Error message: {}", err),
        }
    }

    /// gets user all transations
    pub fn all(conn: &mut PgConnection, user_id: &str) -> DatabaseResult<Vec<Transaction>> {
        use super::schema::transaction::user_id as ui;
        match transaction::table
            .filter(ui.eq(user_id))
            .load::<Transaction>(conn)
        {
            Ok(trans_vec) => DatabaseResult::Succeful(trans_vec),
            Err(err) => panic!("Something went wrong, Error message: {}", err),
        }
    }

    /// gets user specific transaction
    pub fn get(conn: &mut PgConnection, id: i32) -> DatabaseResult<Transaction> {
        use super::schema::transaction::id as i;
        match transaction::table
            .filter(i.eq(id))
            .load::<Transaction>(conn)
        {
            Ok(mut trans_vec) if !trans_vec.is_empty() => {
                DatabaseResult::Succeful(trans_vec.pop().unwrap())
            }
            Err(err) => panic!("Something is wrong: Error message {}", err),
            _ => panic!("Something is wrong, needs maintence!"),
        }
    }

    /// deletes a transaction
    pub fn delete(conn: &mut PgConnection, id: i32) -> DatabaseResult<Transaction> {
        use super::schema::transaction::id as i;
        match diesel::delete(transaction::table.filter(i.eq(id))).get_result::<Transaction>(conn) {
            Ok(trans) => DatabaseResult::Succeful(trans),
            Err(Error::DatabaseError(_, _)) => DatabaseResult::NotFound,
            Err(err) => panic!("Something is wrong, Error message: {}", err),
        }
    }

    /// deletes a user account all transaction
    pub fn delete_account(
        conn: &mut PgConnection,
        account_id: i32,
    ) -> DatabaseResult<Vec<Transaction>> {
        use super::schema::transaction::bank_account as ba;
        match diesel::delete(transaction::table.filter(ba.eq(account_id)))
            .get_results::<Transaction>(conn)
        {
            Ok(trans_vec) => DatabaseResult::Succeful(trans_vec),
            Err(err) => panic!("Something is wrong, Error message: {}", err),
        }
    }
}

#[derive(Debug, Insertable, Clone)]
#[table_name = "transaction"]
pub struct NewTransaction {
    pub kind: bool,
    pub title: Option<String>,
    pub value: String,
    pub currency: CurrencyType,
    pub time: NaiveDate,
    pub user_id: String,
    pub bank_account: i32,
}

impl<'a> NewTransaction {
    fn new(
        kind: bool,
        title: Option<String>,
        value: String,
        currency: CurrencyType,
        user_id: String,
        bank_account: i32,
    ) -> NewTransaction {
        let time = Local::today().naive_local();

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

impl Default for NewTransaction {
    fn default() -> NewTransaction {
        NewTransaction {
            kind: true,
            title: Some("Huh".to_string()),
            value: "344134000".to_string(),
            currency: CurrencyType::USD,
            time: Local::today().naive_local(),
            user_id: "test_user".to_string(),
            bank_account: 1,
        }
    }
}

#[derive(Debug, AsExpression, FromSqlRow, PartialEq, Deserialize, Serialize, Clone)]
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

#[cfg(test)]
mod test {
    use super::super::establish_connection;
    use super::*;

    #[test]
    fn transaction_create() {
        let mut conn = establish_connection();

        let new_trans = NewTransaction::default();
        let NewTransaction {
            kind,
            title,
            value,
            currency,
            time,
            user_id,
            bank_account,
        } = new_trans.clone();

        let query_result = Transaction::add(&mut conn, &new_trans).unwrap();

        let should_match = Transaction::new(
            kind,
            title,
            value,
            currency,
            time,
            user_id,
            query_result.id,
            bank_account,
        );

        assert_eq!(query_result, should_match);

        // cleans up inserted row
        Transaction::delete(&mut conn, query_result.id);
    }

    #[test]
    fn transaction_get() {
        let mut conn = establish_connection();

        let new_trans = NewTransaction::default();
        let NewTransaction {
            kind,
            title,
            value,
            currency,
            time,
            user_id,
            bank_account,
        } = new_trans.clone();

        let query_result = Transaction::add(&mut conn, &new_trans).unwrap();
        let query_result = Transaction::get(&mut conn, query_result.id).unwrap();

        let should_match = Transaction::new(
            kind,
            title,
            value,
            currency,
            time,
            user_id,
            query_result.id,
            bank_account,
        );

        assert_eq!(should_match, query_result);
        // cleans up the added user
        Transaction::delete(&mut conn, query_result.id);
    }

    #[test]
    fn transaction_delete() {
        let mut conn = establish_connection();

        let new_trans = NewTransaction::default();

        // adds the user before the deleting
        let query_res = Transaction::add(&mut conn, &new_trans).unwrap();

        Transaction::delete(&mut conn, query_res.id);
    }
}
