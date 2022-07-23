use financial_managment::*;
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes::stage())
        .mount("/api", api::stage())
}
