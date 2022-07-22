use super::establish_connection;
use super::DatabaseResult;
use crate::authentication::gaurd;
use crate::models::User;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct UserData {
    pub name: String,
    pub username: String,
    pub password: String,
}

/// POST to creating a new user (Admin level)
#[post("/users", format = "application/json", data = "<new_user>")]
pub fn super_create_user(new_user: Json<UserData>, admin: gaurd::AdminGaurd) {}

/// GET to retrieve all users (Admin level)
#[get("/users")]
pub fn super_get_all_user(admin: gaurd::AdminGaurd) {}

/// GET to retrieve a user (Admin level)
#[get("/users/<username>")]
pub fn super_get_user(username: &str, user: gaurd::AdminGaurd) {}

/// PATCH to update a user info (Admin level)
#[patch("/users/<username>")]
pub fn super_update_user(username: &str, user: gaurd::AdminGaurd) {}

/// DELETE to delete a user (Admin level)
#[delete("/users/<username>")]
pub fn super_delete_user(username: &str, user: gaurd::AdminGaurd) {}

/// POST to creating a new user
#[post("/users", format = "application/json", data = "<new_user>")]
pub fn create_user(new_user: Json<UserData>) {}

/// GET to retrieve a user
#[get("/users/<username>")]
pub fn get_user(username: &str, user: gaurd::UserGaurd) {}

/// PATCH to update a user info
#[patch("/users/<username>")]
pub fn update_user(username: &str, user: gaurd::UserGaurd) {}

/// DELETE to delete a user
#[delete("/users/<username>")]
pub fn delete_user(username: &str, user: gaurd::UserGaurd) {}
