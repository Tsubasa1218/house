#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::databases::rusqlite;

#[database("house_db")]
pub struct HouseDBConn(rusqlite::Connection);

use actions::print_test;
mod actions;

mod domains;

fn main() {
    rocket::ignite()
        .attach(HouseDBConn::fairing())
        .mount("/", routes![print_test::index])
        .launch();
}
