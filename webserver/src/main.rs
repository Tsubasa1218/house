#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::databases::rusqlite;

#[database("house_db")]
pub struct HouseDBConn(rusqlite::Connection);

mod actions;
mod domains;
mod responders;

fn main() {
    rocket::ignite()
        .attach(HouseDBConn::fairing())
        .mount(
            "/",
            routes![
                actions::get_measure_types,
                actions::post_measures,
                actions::get_measures
            ],
        )
        .launch();
}
