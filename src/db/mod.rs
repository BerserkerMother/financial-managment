use core::ops::{Deref, DerefMut};
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use dotenv::dotenv;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::State;
use rocket::{request::FromRequest, Request};
use std::env;

type ConnPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbConn(pub PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for DbConn {
    type Target = PooledConnection<ConnectionManager<PgConnection>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for DbConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<DbConn, (Status, ()), ()> {
        let pool = match req.guard::<&State<ConnPool>>().await {
            Outcome::Success(outcome) => outcome,
            _ => return Outcome::Failure((Status::ServiceUnavailable, ())),
        };
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

pub fn get_conn_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).unwrap()
}
