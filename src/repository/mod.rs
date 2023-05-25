use rusqlite::{Connection, Result};

pub mod keys;

pub fn init_connection() -> Result<Connection> {
    let co = Connection::open("test.db")?;
    Ok(co)
}

pub fn init_database_schemas() -> Result<Connection> {
    let co = init_connection()?;
    init_databases(&co)?;
    Ok(co)
}

fn init_databases(co: &Connection) -> Result<()> {
    co.execute(
        "CREATE TABLE IF NOT EXISTS keys (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            value TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
)",
        [],
    )?;

    Ok(())
}
