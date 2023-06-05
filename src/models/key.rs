use chrono::prelude::*;
use log::debug;

#[derive(Debug, Clone)]
pub struct Key {
    id: i64,
    name: String,
    value: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Key {
    pub fn new(
        id: Option<i64>,
        name: String,
        value: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        // let created_at = Utc::now();
        // let updated_at = Utc::now();
        let id = id.unwrap_or(0);

        Self {
            id,
            name,
            value,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn created_at(&self) -> String {
        let tmp = &self.created_at;
        tmp.to_rfc3339()
    }

    pub fn updated_at(&self) -> String {
        self.updated_at.to_rfc3339()
    }

    pub fn persist(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open("keys.db")?;
        conn.execute(
            "INSERT INTO keys (name, value, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![self.name, self.value, self.created_at, self.updated_at],
        )?;
        Ok(())
    }

    pub fn retrive_keys_from_db() -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open("keys.db")?;
        let mut stmt = conn.prepare("SELECT id, name, value, created_at, updated_at FROM keys")?;
        let rows = stmt.query_map(rusqlite::params![], |row| {
            Ok(Key::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        let mut keys = Vec::new();
        for key in rows {
            keys.push(key?);
        }
        Ok(keys)
    }
}
