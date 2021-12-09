pub mod print_test {
    use crate::domains;
    use crate::HouseDBConn;
    use rocket_contrib::databases::rusqlite;

    #[get("/")]
    pub fn index(conn: HouseDBConn) -> rusqlite::Result<String> {
        let result = domains::select(
            &*conn,
            domains::SimpleSelect {
                table: "test",
                fields: vec!["rowid", "name"],
            },
        )?;

        let mut response = String::from("[");

        for r in result {
            response.push('{');

            for (key, value) in r {
                let value_value = match value {
                    rusqlite::types::Value::Text(s) => s,
                    rusqlite::types::Value::Integer(i) => i.to_string(),
                    _=> String::new()
                } ;
                response.push_str(&key);
                response.push_str(": ");
                response.push_str(value_value.as_str());
                response.push_str(", ");
            }

            response.push_str("}, ");
        }

        response.push(']');

        Ok(response)
    }
}
