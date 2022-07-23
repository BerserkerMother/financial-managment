pub mod api;
pub mod authentication;
pub mod models;
pub mod routes;
mod schema;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
struct Test(i32, String);

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
