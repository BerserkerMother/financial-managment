use super::establish_connection;
use super::DatabaseResult;
use crate::authentication::gaurd;
use crate::authentication::hasher::Hash;
use crate::models::User;
use crate::DbConn;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct UserData {
    pub name: String,
    pub username: String,
    pub password: String,
}

/// GET to retrieve all users (Admin level)
#[get("/admin/users")]
pub fn super_get_all_user(admin: gaurd::AdminGaurd, mut conn: DbConn) -> Option<Json<Vec<User>>> {
    match User::all(&mut conn) {
        DatabaseResult::Succeful(user_vec) => Some(Json(user_vec)),
        _ => None,
    }
}

/// GET to retrieve a user (Admin level)
#[get("/admin/users?<username>")]
pub fn super_get_user(
    username: &str,
    user: gaurd::AdminGaurd,
    mut conn: DbConn,
) -> Option<Json<User>> {
    match User::get(&mut conn, username) {
        DatabaseResult::Succeful(user) => Some(Json(user)),
        _ => None,
    }
}

/// PATCH to update a user info (Admin level)
#[patch(
    "/admin/users?<username>",
    format = "application/json",
    data = "<update>"
)]
pub fn super_update_user(
    update: Json<User>,
    username: &str,
    user: gaurd::AdminGaurd,
    mut conn: DbConn,
) -> Option<Json<User>> {
    let mut update = update.0;
    update.password = update.password.hash();
    if !username.eq(&update.username) {
        return None;
    }
    match User::update(&mut conn, &update) {
        DatabaseResult::Succeful(user) => Some(Json(user)),
        _ => None,
    }
}

/// DELETE to delete a user (Admin level)
#[delete("/admin/users?<username>")]
pub fn super_delete_user(
    username: &str,
    admin: gaurd::AdminGaurd,
    mut conn: DbConn,
) -> Option<Json<User>> {
    match User::delete_by_username(&mut conn, username) {
        DatabaseResult::Succeful(user) => Some(Json(user)),
        _ => None,
    }
}

/// POST to creating a new user
#[post("/users", format = "application/json", data = "<new_user>")]
pub fn create_user(new_user: Json<UserData>, mut conn: DbConn) -> Option<Json<User>> {
    let new_user = new_user.0;
    match User::add(&mut conn, &new_user.into()) {
        DatabaseResult::Succeful(user) => Some(Json(user)),
        _ => None,
    }
}

/// GET to retrieve a user
#[get("/users")]
pub fn get_user(user: gaurd::UserGaurd, mut conn: DbConn) -> Option<Json<User>> {
    match User::get(&mut conn, &user.username) {
        DatabaseResult::Succeful(user) => Some(Json(user)),
        _ => None,
    }
}

/// PATCH to update a user info
#[patch("/users", format = "application/json", data = "<update>")]
pub fn update_user(
    update: Json<User>,
    user: gaurd::UserGaurd,
    mut conn: DbConn,
) -> Option<Json<User>> {
    let mut update = update.0;
    update.password = update.password.hash();
    match User::update(&mut conn, &update) {
        DatabaseResult::Succeful(user) => Some(Json(user)),
        _ => None,
    }
}

/// DELETE to delete a user
#[delete("/users")]
pub fn delete_user(user: gaurd::UserGaurd, mut conn: DbConn) -> Option<Json<User>> {
    let username = &user.username;
    match User::delete_by_username(&mut conn, username) {
        DatabaseResult::Succeful(user) => Some(Json(user)),
        _ => None,
    }
}
