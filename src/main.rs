mod routes;
mod models;
mod controllers;

#[macro_use]
extern crate rocket;

use routes::v1::{create_user, delete_user, get_user_by_id, update_user, get_all_users};
use controllers::mongodb_controller::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/v1/", routes![create_user])
        .mount("/v1/", routes![get_user_by_id])
        // .mount("/v1/", routes![get_user_by_name])
        .mount("/v1/", routes![update_user])
        .mount("/v1/", routes![delete_user])
        .mount("/v1/", routes![get_all_users])
}
