use rocket_contrib::databases::rusqlite;

pub trait SelectBuilder {
    fn build_query(&self) -> String;
}

pub struct SimpleSelect {
    pub table: &'static str,
    pub fields: Vec<&'static str>,
}

impl SelectBuilder for SimpleSelect {
    fn build_query(&self) -> String {
        format!("SELECT {} FROM {};", self.fields.join(", "), self.table)
    }
}

pub fn select(
    conn: &rusqlite::Connection,
    sql_select: SimpleSelect,
) -> rusqlite::Result<Vec<Vec<(String, rusqlite::types::Value)>>> {
    let query = sql_select.build_query();
    let mut stmt = conn.prepare(query.as_str())?;
    let names = stmt.column_names().into_iter().map(|s| String::from(s)).collect::<Vec<_>>();

    let mut rows = stmt.query(&[])?;

    let mut result = Vec::new();

    while let Some(row) = rows.next() {
        let row = row.unwrap();

        let mut all_columns = Vec::new();
        for name in names.iter() {
            all_columns.push((name.clone(), row.get::<_, rusqlite::types::Value>(name.as_ref())));
        }

        result.push(all_columns);
    }

    Ok(result)
}
