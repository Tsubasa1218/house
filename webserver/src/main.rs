#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::rusqlite;

#[database("house_db")]
struct HouseDBConn(rusqlite::Connection);


fn print_test(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM test")?;
    let rows = stmt.query_map(&[], |row| row.get(0))?;

    let mut names = Vec::new();
        for name_result in rows {
        names.push(name_result?);
    }


    Ok(names)
}

#[get("/")]
fn index(conn: HouseDBConn) -> rusqlite::Result<String> {
    let result = print_test(&*conn)?;

    Ok(result.join("-"))
}

fn main() {
    rocket::ignite().attach(HouseDBConn::fairing()).mount("/", routes![index]).launch();

}
