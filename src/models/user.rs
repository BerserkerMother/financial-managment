use super::schema::users;
use super::*;

#[derive(Queryable, Debug, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
#[primary_key(username)]
pub struct User {
    pub name: String,
    pub username: String,
    pub password: String,
    pub api_token: String,
    pub role: bool,
}

// TODO: Update NewUser to match User!!!
impl User {
    /// constructor for User
    fn new(username: &str, password: &str, name: &str) -> User {
        User {
            username: String::from(username),
            password: String::from(password),
            name: String::from(name),
            api_token: String::from("f"),
            role: false,
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
    /// gets a user by its bearer token
    pub fn get_by_token(conn: &mut PgConnection, token: &str) -> DatabaseResult<User> {
        use super::schema::users::api_token as ap;
        let user_vec = users::table.filter(ap.eq(token)).load::<User>(conn);
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

    /// update a user account
    pub fn update(conn: &mut PgConnection, user: &User) -> DatabaseResult<User> {
        use super::schema::users::username as un;
        match diesel::update(users::table.filter(un.eq(&user.username)))
            .set(user)
            .get_result::<User>(conn)
        {
            Ok(user) => DatabaseResult::Succeful(user),
            Err(Error::DatabaseError(_, _)) => DatabaseResult::NotFound,
            _ => panic!("Something went wrong"),
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
    api_token: String,
}

impl NewUser {
    pub fn new(name: String, username: String, password: String) -> NewUser {
        use crate::authentication::random_token;
        let api_token = random_token();
        NewUser {
            name,
            username,
            password,
            api_token,
        }
    }
}

use super::super::api::user::UserData;
impl From<UserData> for NewUser {
    fn from(user: UserData) -> NewUser {
        use crate::authentication::hasher::Hash;
        let UserData {
            name,
            username,
            password,
        } = user;
        User::new_user(username, password.hash(), name)
    }
}

impl<'a> Default for NewUser {
    fn default() -> NewUser {
        NewUser {
            name: String::from("Kimia"),
            username: String::from("absolute_trash"),
            password: String::from("huh"),
            api_token: "f".to_string(),
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
            ..
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
            ..
        } = &new_user;

        User::delete_by_username(&mut conn, &new_user.username);
        User::add(&mut conn, &new_user);
        let query_result = User::get(&mut conn, &new_user.username).unwrap();

        let should_match = User::new(username, password, name);

        assert_eq!(should_match, query_result);
        // cleans up the added user
        User::delete_by_username(&mut conn, &new_user.username);
    }
    #[test]
    fn user_update() {
        let mut conn = establish_connection();

        let new_user = NewUser::default();
        User::delete_by_username(&mut conn, &new_user.username);
        if let DatabaseResult::Succeful(mut user) = User::add(&mut conn, &new_user) {
            user.name = String::from("Changed");
            println!("{:?}", user);
            let updated = match User::update(&mut conn, &user) {
                DatabaseResult::Succeful(user) => user,
                _ => panic!("WTF"),
            };
        }
        let query_result = User::get(&mut conn, &new_user.username).unwrap().name;

        let should_match = String::from("Changed");

        assert_eq!(should_match, query_result);
    }
}
