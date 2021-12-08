 pub mod print_test {
     use rocket_contrib::databases::rusqlite;
 
     pub fn work(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<String>> {
         let mut stmt = conn.prepare("SELECT name FROM test")?;
         let rows = stmt.query_map(&[], |row| row.get(0))?;
 
         let mut names = Vec::new();
         for name_result in rows {
             names.push(name_result?);
         }
 
         Ok(names)
     }
 }


