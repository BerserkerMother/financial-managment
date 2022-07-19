use super::schema::account;
use super::*;

#[derive(Queryable, Debug, PartialEq)]
pub struct Account {
    pub balance: Option<String>,
    pub user_id: String,
    pub id: i32,
    pub name: String,
}

impl Account {
    /// Constructor for Account
    fn new(balance: Option<&str>, user_id: &str, id: i32, name: &str) -> Account {
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

    /// creates a NewAcount
    pub fn new_account<'a>(name: &'a str, user_id: &'a str) -> NewAccount<'a> {
        NewAccount::new(user_id, name)
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
    fn new(user_id: &'a str, name: &'a str) -> NewAccount<'a> {
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

#[cfg(test)]
mod test {
    use super::super::establish_connection;
    use super::*;

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
