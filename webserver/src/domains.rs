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
    use chrono::{DateTime, Utc};
    use rocket_contrib::databases::rusqlite;
    use rocket_contrib::json::Json;
    use std::time;

    fn current_date() -> String {
        let system_time = time::SystemTime::now();
        let date_time: DateTime<Utc> = system_time.into();

        date_time.to_rfc3339()
    }

    pub fn insert_measurement(
        conn: &rusqlite::Connection,
        measure_json: Json<actions::MeasureRecord>,
    ) -> rusqlite::Result<()> {
        let measure_types = measure_type::select_measure_types(&conn)?;

        let mut stmt = conn.prepare(
            "INSERT INTO measure (time, measure, measure_type_id) VALUES (:time, :measure, :measure_type)"
        )?;

        let measure = measure_json.into_inner();
        let measure_key_tuples = measure_types
            .iter()
            .map(|m_type| match m_type.name.as_str() {
                "pm2.5" => (m_type.id, measure.pm02.to_string()),
                "co2" => (m_type.id, measure.rco2.to_string()),
                "temperature" => (m_type.id, measure.atmp.to_string()),
                "humidity" => (m_type.id, measure.rhum.to_string()),
                _ => (-1, String::new()),
            });

        let date_time = current_date();

        for (measure_type_id, measure_value) in measure_key_tuples {
            if measure_type_id != -1 {
                stmt.execute_named(&[
                    (":time", &date_time),
                    (":measure", &measure_value),
                    (":measure_type", &measure_type_id),
                ])?;
            }
        }

        Ok(())
    }
}
