use super::schema::users;
use super::*;

#[derive(Queryable, Debug, PartialEq, Serialize)]
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
    pub fn new_user(username: String, password: String, name: String) -> NewUser {
        NewUser::new(name, username, password)
    }

    /// gets a user from database with id
    /// if the there exist a user with given id returns DatabaseResult::Successful(User)
    ///
    /// Otherwise returns DatabaseResult::NotFound
    pub fn get(conn: &mut PgConnection, username: &str) -> DatabaseResult<User> {
        use super::schema::users::username as u;
        let user_vec = users::table.filter(u.eq(username)).load::<User>(conn);
        match user_vec {
            Ok(mut user_vec) => {
                if user_vec.is_empty() {
                    DatabaseResult::NotFound
                } else {
                    DatabaseResult::Succeful(user_vec.pop().unwrap())
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
    /// if the user already exits returns DatabaseResult::AlreadyExists
    /// otherwise DatabaseResults::Successful(User)
    /// # panics
    /// Panics due to unknown error!
    pub fn add(conn: &mut PgConnection, new_user: &NewUser) -> DatabaseResult<User> {
        match diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(conn)
        {
            Ok(inserted_user) => DatabaseResult::Succeful(inserted_user),
            Err(Error::DatabaseError(_, _)) => DatabaseResult::AlreadyExists,
            Err(err) => panic!(
                "Something went wrong while inserting data, Error message: {}",
                err
            ),
        }
    }

    /// deletes a user by its username and returns it
    /// if user doesn't exits it returns DatabaseResult::NotFound
    pub fn delete_by_username(conn: &mut PgConnection, username: &str) -> DatabaseResult<User> {
        use super::schema::users::username as u;

        match diesel::delete(users::table.filter(u.eq(username))).get_result::<User>(conn) {
            Ok(new_user) => DatabaseResult::Succeful(new_user),
            Err(Error::NotFound) => DatabaseResult::NotFound,
            Err(err) => panic!(
                "Something went wrong while deleting data, Error message: {}",
                err
            ),
        }
    }
}

use result_variant::DatabaseAletr;
impl DatabaseAletr for User {}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    name: String,
    username: String,
    password: String,
}

impl NewUser {
    pub fn new(name: String, username: String, password: String) -> NewUser {
        NewUser {
            name,
            username,
            password,
        }
    }
}

use super::super::api::models::UserData;
impl From<UserData> for NewUser {
    fn from(user: UserData) -> NewUser {
        let UserData {
            name,
            username,
            password,
        } = user;
        User::new_user(username, password, name)
    }
}

impl<'a> Default for NewUser {
    fn default() -> NewUser {
        NewUser {
            name: String::from("Kimia"),
            username: String::from("absolute_trash"),
            password: String::from("huh"),
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
        } = &new_user;

        // makes sure the user doesn't exits
        User::delete_by_username(&mut conn, &username);

        let query_result = User::add(&mut conn, &new_user).unwrap();

        let should_match = User::new(username, password, name);

        assert_eq!(query_result, should_match);

        // cleans up inserted row
        User::delete_by_username(&mut conn, &username);
    }

    #[test]
    fn user_delete() {
        let mut conn = establish_connection();

        let new_user = NewUser::default();
        let username = &new_user.username;

        // adds the user before the deleting
        User::add(&mut conn, &new_user);

        User::delete_by_username(&mut conn, username);
    }

    #[test]
    fn user_get() {
        let mut conn = establish_connection();

        let new_user = NewUser::default();
        let NewUser {
            username,
            password,
            name,
        } = &new_user;

        User::delete_by_username(&mut conn, &new_user.username);
        User::add(&mut conn, &new_user);
        let query_result = User::get(&mut conn, &new_user.username).unwrap();

        let should_match = User::new(username, password, name);

        assert_eq!(should_match, query_result);
        // cleans up the added user
        User::delete_by_username(&mut conn, &new_user.username);
    }
}
