use anyhow::Result as AnyResult;
use rusqlite::{Connection, Result};
pub fn establish_connection() -> AnyResult<Connection> {
    let conn = Connection::open("test_db/dummy.db")?;
    Ok(conn)
}
