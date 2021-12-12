pub mod measure_type {
    use rocket_contrib::databases::rusqlite;

    #[derive(Debug, Serialize)]
    pub struct MeasureType {
        pub id: isize,
        pub name: String,
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

pub mod measures {
    use super::measure_type;
    use crate::actions;
    use rocket_contrib::databases::rusqlite;
    use rocket_contrib::json::Json;
    use std::time;
    use chrono::{DateTime, Utc};

    pub fn insert_measurement(
        conn: &rusqlite::Connection,
        measure_json: Json<actions::MeasureRecord>,
    ) -> rusqlite::Result<()> {
        let measure_types = measure_type::select_measure_types(&conn)?;
        let system_time = time::SystemTime::now();
        let date_time : DateTime<Utc>= system_time.into().format("%+");

        let measure = measure_json.into_inner();

        let stmt = conn.prepare(
            "INSERT INTO measure (time, measure, measure_type_id) VALUES (:time, :measure, :measure_type)"
        )?;

        let types_iter = measure_types.iter();

        let pm_id = match types_iter.find(|&&m_type| m_type.name == "pm2.5") {
            Some(measure_type) => measure_type.id,
            _ => -1,
        };

        if pm_id != -1 {
            stmt.execute_named(&[(":time", &date_time), ("measure", &measure.pm02), ("measure_id", &pm_id)])?;
        }

        Ok(())
    }
}
