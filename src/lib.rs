mod models;
mod schema;

#[macro_use]
extern crate diesel;

use self::schema::{account, transaction, users};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::error::Error;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{NewUser, User};

/// creates a new user in database
fn create_user(
    conn: &mut PgConnection,
    name: &str,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    use self::schema::users;

    let new_user = NewUser::new(name, username, password);

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)?;

    Ok(())
}

use self::models::CurrencyType;
use self::models::{NewTransaction, Transaction};

fn create_transaction(
    conn: &mut PgConnection,
    kind: bool,
    source: Option<&str>,
    note: Option<&str>,
    value: &str,
    currency: Option<CurrencyType>,
    time: (i32, u32, u32),
    user_id: &str,
    bank_account: Option<i32>,
) -> Result<(), Box<dyn Error>> {
    let new_transaction = NewTransaction::new(
        kind,
        source,
        note,
        value,
        currency,
        time,
        user_id,
        bank_account,
    );

    diesel::insert_into(transaction::table)
        .values(&new_transaction)
        .execute(conn)?;
    Ok(())
}

use self::models::{Account, NewAccount};

fn create_account(
    conn: &mut PgConnection,
    name: &str,
    user_id: &str,
) -> Result<(), Box<dyn Error>> {
    let new_account = NewAccount::new(user_id, name);

    diesel::insert_into(account::table)
        .values(&new_account)
        .execute(conn)?;
    Ok(())
}

// write some test

pub fn functionality_test() {
    // let mut conn = establish_connection();
}

#[cfg(test)]
mod test {
    use self::schema::account::dsl::{account as all_account, id as acc_id};
    use self::schema::transaction::dsl::{id as trans_id, transaction as all_transaction};
    use self::schema::users::dsl::{username, users as all_users};
    use super::*;
    #[test]
    fn transaction_create() {
        let new_transaction = NewTransaction::default();
        let mut conn = establish_connection();

        let query_result = diesel::insert_into(transaction::table)
            .values(&new_transaction)
            .get_result::<Transaction>(&mut conn)
            .expect("Test Failed, Can not insert to database");

        let should_match = Transaction {
            kind: true,
            source: Some(String::from("Huh")),
            note: Some(String::from("aksdj kaskjd")),
            value: String::from("344134000"),
            currency: Some(CurrencyType::USD),
            time: chrono::NaiveDate::from_ymd(2000, 01, 01),
            user_id: String::from("BerserkerMother"),
            bank_account: None,
            id: query_result.id,
        };

        assert_eq!(query_result, should_match);

        // cleans up the inserted row
        diesel::delete(all_transaction.filter(trans_id.eq(query_result.id)))
            .execute(&mut conn)
            .expect("Failed to delete the inserted row!");
    }

    #[test]
    fn user_create() {
        let mut conn = establish_connection();

        let new_user = NewUser::default();
        let query_result = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .expect("Test Failed, Can not insert to database");

        let should_match = User {
            username: String::from(new_user.username),
            password: String::from(new_user.password),
            name: String::from(new_user.name),
        };

        assert_eq!(query_result, should_match);

        // cleans up inserted row
        diesel::delete(all_users.filter(username.eq(should_match.username)))
            .execute(&mut conn)
            .expect("Failed to delete the inserted row!");
    }

    #[test]
    fn account_create() {
        let mut conn = establish_connection();

        let new_account = NewAccount::default();
        let query_result = diesel::insert_into(account::table)
            .values(&new_account)
            .get_result::<Account>(&mut conn)
            .expect("Test Failed, Can not insert into database!");

        let should_match = Account {
            user_id: String::from(new_account.user_id),
            id: query_result.id,
            balance: Some(String::from(new_account.balance.unwrap())),
            name: String::from(new_account.name),
        };

        assert_eq!(query_result, should_match);

        // cleans up inserted row
        diesel::delete(all_account.filter(acc_id.eq(should_match.id)))
            .execute(&mut conn)
            .expect("Failed to delete the inserted row!");
    }
}
