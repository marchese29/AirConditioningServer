#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use crate::database::ConditionDbConn;
use crate::mounts::events::*;
use crate::mounts::relations::*;

mod database;
mod errors;
mod logic;
mod model;
mod mounts;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(ConditionDbConn::fairing())
        .mount("/", routes![index])
        .mount("/events", routes![set_condition, unset_condition])
        .mount(
            "/relations",
            routes![
                create_trigger,
                create_new_condition,
                list_triggers,
                list_conditions,
                describe_trigger
            ],
        )
        .mount("/registry", routes![])
        .launch()
        .await;
}
