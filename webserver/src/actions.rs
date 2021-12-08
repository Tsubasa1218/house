
pub mod print_test {
    use crate::HouseDBConn;
    use rocket_contrib::databases::rusqlite;
    use crate::domains;

    #[get("/")]
    pub fn index(conn: HouseDBConn) -> rusqlite::Result<String> {
        let result = domains::print_test::work(&*conn)?;

        Ok(result.join("-"))
    }
}



