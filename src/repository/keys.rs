use rusqlite::Error;

use super::init_connection;
use crate::models::key::Key;

pub fn retrive_keys_from_db() -> Result<Vec<Key>, Error> {
    let co = init_connection()?;

    let mut stmt = co.prepare("SELECT id, name, value, created_at, updated_at FROM keys")?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Key::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })
        .unwrap();

    let mut keys = Vec::new();
    for key in rows {
        keys.push(key?);
    }
    Ok(keys)
}

pub fn insert_key_to_db(key: &Key) -> Result<(), Error> {
    let co = init_connection()?;

    co.execute(
        "INSERT INTO keys (name, value) VALUES (?1, ?2)",
        &[&key.name(), &key.value()],
    )?;
    Ok(())
}
