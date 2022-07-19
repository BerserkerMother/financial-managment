use super::schema::users;
use super::*;

#[derive(Queryable, Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub username: String,
    pub password: String,
}

impl User {
    /// constructor for User
    fn new(username: &str, password: &str, name: &str) -> User {
        User {
            username: String::from(username),
            password: String::from(password),
            name: String::from(name),
        }
    }

    /// creates a NewUser
    pub fn new_user<'a>(username: &'a str, password: &'a str, name: &'a str) -> NewUser<'a> {
        NewUser::new(name, username, password)
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
}
