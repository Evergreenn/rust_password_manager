use super::init_connection;
use crate::models::key::Key;
use rusqlite::Error;

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

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}
