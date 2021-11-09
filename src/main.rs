#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use crate::database::ConditionDbConn;

mod database;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(ConditionDbConn::fairing())
        .mount("/", routes![index])
        .launch()
        .await;
}
