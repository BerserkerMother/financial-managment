use financial_managment::*;
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![get_user, create_user, redirect, loggin, loggin_page],
    )
}
