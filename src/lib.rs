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

pub fn functionality_test() {
    // let mut conn = establish_connection();
}
