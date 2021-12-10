pub mod measure_type {
    use rocket_contrib::databases::rusqlite;

    #[derive(Debug, Serialize)]
    pub struct MeasureType {
        id: isize,
        name: String,
    }

    pub fn select_measure_types(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<MeasureType>> {
        let mut stmt = conn.prepare("SELECT rowid, name FROM measure_type")?;
        let rows = stmt.query_map(&[], |row| MeasureType {
            id: row.get(0),
            name: row.get(1),
        })?;

        let mut measure_types = Vec::new();

        for row in rows {
            measure_types.push(row?);
        }

        Ok(measure_types)
    }
}
