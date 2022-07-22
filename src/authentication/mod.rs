use rocket::http::{Cookie, CookieJar};
use rocket::outcome::{IntoOutcome, Outcome};
use rocket::request::{self, FromRequest, Request};
use std::convert::Infallible;

pub mod gaurd {
    use super::*;
    #[derive(Debug)]
    pub struct AdminGaurd(String);

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for AdminGaurd {
        type Error = Infallible;

        async fn from_request(req: &'r Request<'_>) -> request::Outcome<AdminGaurd, Self::Error> {
            req.cookies()
                .get_private("username")
                .and_then(|coockie| Some(String::from(coockie.value())))
                .map(|s| AdminGaurd(s))
                .or_forward(())
        }
    }

    #[derive(Debug)]
    pub struct UserGaurd(String);

    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for UserGaurd {
        type Error = Infallible;

        async fn from_request(req: &'r Request<'_>) -> request::Outcome<UserGaurd, Self::Error> {
            Outcome::Success(UserGaurd(String::from("BerserkerMother")))
        }
    }
}
