use chrono::NaiveDate;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::*;
use diesel::result::Error;
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

impl Account {
    /// Constructor for Account
    pub fn new(balance: Option<&str>, user_id: &str, id: i32, name: &str) -> Account {
        Account {
            balance: match balance {
                Some(bal) => Some(String::from(bal)),
                None => None,
            },
            user_id: String::from(user_id),
            id: id,
            name: String::from(name),
        }
    }
    /// gets a bank account from acount table
    ///
    /// returns Some(Account) if it exists
    ///
    /// returns None if there is no such data
    pub fn get(conn: &mut PgConnection, name: &str, user_id: &str) -> Option<Account> {
        use super::schema::account::{name as n, user_id as ui};
        match account::table
            .filter(ui.eq(user_id))
            .filter(n.eq(name))
            .load::<Account>(conn)
        {
            Ok(acc_vec) if acc_vec.is_empty() => None,
            Ok(mut acc_vec) => acc_vec.pop(),
            Err(e) => panic!(
                "Something went wrong while getting data!, Error message: {}",
                e
            ),
        }
    }

    /// adds bank account to account table
    ///
    /// returns Some(Account) if acount doesn't exists
    ///
    /// returns None if Account already exist
    pub fn add(conn: &mut PgConnection, new_account: &NewAccount) -> Option<Account> {
        match diesel::insert_into(account::table)
            .values(new_account)
            .get_result::<Account>(conn)
        {
            Ok(acc) => Some(acc),
            Err(Error::DatabaseError(_, _)) => None,
            Err(err) => panic!(
                "Something went wrong while inserting data, Error message: {}",
                err
            ),
        }
    }

    /// delete a bank account from account
    /// return Some(Account) if account is successfully deleted
    ///
    /// returns None if there is no such account by id
    pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Option<Account> {
        use super::schema::account::id as i;
        match diesel::delete(account::table.filter(i.eq(id))).get_result::<Account>(conn) {
            Ok(new_acc) => Some(new_acc),
            Err(Error::NotFound) => None,
            Err(err) => panic!(
                "Something went wrong while deleting data, Error message: {}",
                err
            ),
        }
    }

    /// delete a back account with user_id and name
    ///
    /// returns Some(Account) if the account exists and
    /// it's successfully deleted. If the account doesn't
    /// exists function returns None
    pub fn delete_by_name_user(
        conn: &mut PgConnection,
        name: &str,
        user_id: &str,
    ) -> Option<Account> {
        use super::schema::account::{name as n, user_id as ui};
        match diesel::delete(account::table)
            .filter(ui.eq(user_id))
            .filter(n.eq(name))
            .get_result::<Account>(conn)
        {
            Ok(acc) => Some(acc),
            Err(Error::NotFound) => None,
            Err(err) => panic!(
                "Some went wrong while deleteing the data, Error message {}",
                err
            ),
        }
    }
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

impl User {
    /// constructor for User
    pub fn new(username: &str, password: &str, name: &str) -> User {
        User {
            username: String::from(username),
            password: String::from(password),
            name: String::from(name),
        }
    }

    /// gets a user from database with id
    /// returns Option<User> if user exits
    ///
    /// returns None if there is no such user
    pub fn get(conn: &mut PgConnection, username: &str) -> Option<User> {
        use super::schema::users::username as u;
        let user_vec = users::table.filter(u.eq(username)).load::<User>(conn);
        match user_vec {
            Ok(mut user_vec) => {
                if user_vec.is_empty() {
                    None
                } else {
                    user_vec.pop()
                }
            }
            Err(err) => panic!(
                "Something went wrong while getting data, Error message: {}",
                err
            ),
        }
    }

    /// inserts a new user to users table
    ///
    /// if the user already exits returns None
    /// otherwise Some(User)
    /// # panics
    /// Panics due to unknown error!
    pub fn add(conn: &mut PgConnection, new_user: &NewUser) -> Option<User> {
        match diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(conn)
        {
            Ok(inserted_user) => Some(inserted_user),
            Err(Error::DatabaseError(_, _)) => None,
            Err(err) => panic!(
                "Something went wrong while inserting data, Error message: {}",
                err
            ),
        }
    }

    /// deletes a user by its username and returns it
    /// if user doesn't exits it returns None
    pub fn delete_by_username(conn: &mut PgConnection, username: &str) -> Option<User> {
        use super::schema::users::username as u;

        match diesel::delete(users::table.filter(u.eq(username))).get_result::<User>(conn) {
            Ok(new_user) => Some(new_user),
            Err(Error::NotFound) => None,
            Err(err) => panic!(
                "Something went wrong while deleting data, Error message: {}",
                err
            ),
        }
    }
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

#[cfg(test)]
mod test {
    use super::super::establish_connection;
    use super::*;
    #[test]
    fn user_create() {
        let mut conn = establish_connection();

        let new_user = NewUser::default();
        let NewUser {
            username,
            password,
            name,
        } = new_user;

        // makes sure the user doesn't exits
        User::delete_by_username(&mut conn, username);

        let query_result = User::add(&mut conn, &new_user).unwrap();

        let should_match = User::new(username, password, name);

        assert_eq!(query_result, should_match);

        // cleans up inserted row
        User::delete_by_username(&mut conn, username);
    }

    #[test]
    fn user_delete() {
        let mut conn = establish_connection();

        let new_user = NewUser::default();
        let username = new_user.username;

        // adds the user before the deleting
        User::add(&mut conn, &new_user);

        if let None = User::delete_by_username(&mut conn, username) {
            panic!("Error while deleting user, User doesn't exits!!!")
        }
    }

    #[test]
    fn user_get() {
        let mut conn = establish_connection();

        let new_user = NewUser::default();
        let NewUser {
            username,
            password,
            name,
        } = new_user;

        User::delete_by_username(&mut conn, new_user.username);
        User::add(&mut conn, &new_user);
        let query_result = User::get(&mut conn, new_user.username).unwrap();

        let should_match = User::new(username, password, name);

        assert_eq!(should_match, query_result);
        // cleans up the added user
        User::delete_by_username(&mut conn, &new_user.username);
    }
    #[test]
    fn account_create() {
        let mut conn = establish_connection();

        let new_account = NewAccount::default();
        let NewAccount {
            balance,
            user_id,
            name,
        } = new_account;

        let query_result = Account::add(&mut conn, &new_account).unwrap();

        let should_match = Account::new(balance, user_id, query_result.id, name);

        assert_eq!(query_result, should_match);

        // cleans up inserted row
        Account::delete_by_id(&mut conn, query_result.id);
    }

    #[test]
    fn account_delete() {
        let mut conn = establish_connection();

        let new_account = NewAccount::default();
        let NewAccount { user_id, name, .. } = new_account;
        Account::add(&mut conn, &new_account);

        match Account::delete_by_name_user(&mut conn, name, user_id) {
            Some(_) => {}
            None => panic!("Test failed, no data was deleted!"),
        }
    }

    #[test]
    fn account_get() {
        let mut conn = establish_connection();

        let new_account = NewAccount::default();
        let NewAccount {
            name,
            balance,
            user_id,
        } = new_account;

        Account::add(&mut conn, &new_account);

        let query_result = Account::get(&mut conn, name, user_id).unwrap();

        let should_match = Account::new(balance, user_id, query_result.id, name);

        assert_eq!(query_result, should_match);

        // cleans up added data
        Account::delete_by_name_user(&mut conn, name, user_id);
    }
}
