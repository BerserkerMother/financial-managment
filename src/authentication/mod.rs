pub mod hasher;
pub mod token_generator;
use crate::establish_connection;
pub use token_generator::random_token;

use crate::models::result_variant::DatabaseResult;
use crate::models::User;
use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::outcome::{IntoOutcome, Outcome};
use rocket::request::{self, FromRequest, Request};
use std::convert::Infallible;

#[derive(Debug)]
pub enum GaurdError {
    NotAdmin,
}

pub mod gaurd {
    use super::*;
    #[derive(Debug)]
    pub struct AdminGaurd {
        pub username: String,
    }

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for AdminGaurd {
        type Error = GaurdError;

        async fn from_request(req: &'r Request<'_>) -> request::Outcome<AdminGaurd, Self::Error> {
            let mut values: Vec<_> = req.headers().get("Authorization").collect();
            if values.len() != 1 {
                return Outcome::Forward(());
            }
            let mut header_fileds = values[0].split(' ');
            match header_fileds.next() {
                Some(method) if method == "Bearer" => (),
                _ => return Outcome::Forward(()),
            }
            let token = match header_fileds.next() {
                Some(token) => token,
                _ => return Outcome::Forward(()),
            };

            let mut conn = establish_connection();
            match User::get_by_token(&mut conn, token) {
                DatabaseResult::Succeful(user) if user.role == true => {
                    Outcome::Success(AdminGaurd {
                        username: user.username,
                    })
                }
                _ => return Outcome::Failure((Status::BadRequest, GaurdError::NotAdmin)),
            }
        }
    }

    #[derive(Debug)]
    pub struct UserGaurd {
        pub username: String,
    }
    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for UserGaurd {
        type Error = Infallible;

        async fn from_request(req: &'r Request<'_>) -> request::Outcome<UserGaurd, Self::Error> {
            let mut values: Vec<_> = req.headers().get("Authorization").collect();
            if values.len() != 1 {
                return Outcome::Forward(());
            }
            let mut values = values[0].split(' ');
            match values.next() {
                Some(method) if method == "Bearer" => (),
                _ => return Outcome::Forward(()),
            };

            let token = match values.next() {
                Some(token) => token,
                _ => return Outcome::Forward(()),
            };

            let mut conn = establish_connection();
            match User::get_by_token(&mut conn, token) {
                DatabaseResult::Succeful(user) => Outcome::Success(UserGaurd {
                    username: user.username,
                }),
                _ => Outcome::Forward(()),
            }
        }
    }
}
