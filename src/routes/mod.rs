use crate::authentication::hasher::Hash;
use crate::establish_connection;
use crate::models::result_variant::DatabaseResult;
use crate::models::User;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credential {
    username: String,
    password: String,
}

#[get("/")]
pub fn to_loging() -> Redirect {
    Redirect::to(uri!(login))
}

#[get("/login", format = "application/json", data = "<credential>")]
pub fn login(credential: Json<Credential>) -> Option<Json<User>> {
    let mut conn = establish_connection();
    let credential = credential.0;
    let user = match User::get(&mut conn, &credential.username) {
        DatabaseResult::Succeful(user) => user,
        _ => return None,
    };
    if credential.password.hash().eq(&user.password) {
        Some(Json(user))
    } else {
        None
    }
}

use rocket::Route;
pub fn stage() -> Vec<Route> {
    routes![to_loging, login]
}
