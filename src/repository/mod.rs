use rusqlite::{Connection, Result};

pub fn init_connection() -> Result<Connection> {
    let co = Connection::open("keys.db")?;
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
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            last_used_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            last_changed_at DATETIME DEFAULT CURRENT_TIMESTAMP

)",
        [],
    )?;

    //     co.execute(
    //         "CREATE TABLE IF NOT EXISTS passwords (
    //             id TEXT PRIMARY KEY,
    //             password TEXT NOT NULL,
    //             created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    //             updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    //             last_used_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    //             last_changed_at DATETIME DEFAULT CURRENT_TIMESTAMP
    // )",
    //         [],
    //     )?;
    Ok(())
}
