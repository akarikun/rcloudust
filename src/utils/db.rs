use rusqlite::{params, Connection, Result};

pub struct DB{
    
}
impl DB{
    pub fn get_conn() -> Result<Connection, rusqlite::Error> {
        let conn = Connection::open("./data.db")?;
        Ok(conn)
    }
}