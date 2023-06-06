use chrono::prelude::*;
use passwords::PasswordGenerator;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Key {
    id: Uuid,
    name: String,
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_used_at: DateTime<Utc>,
    last_changed_at: DateTime<Utc>,
}

impl Key {
    pub fn new(id: Option<Uuid>, name: String) -> Self {
        let id = id.unwrap_or(Uuid::new_v4());
        let now = Utc::now();
        let pg = PasswordGenerator::new()
            .length(32)
            .numbers(true)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(true)
            .spaces(false)
            .exclude_similar_characters(true)
            .strict(true);
        let password = pg.generate_one().unwrap();

        Self {
            id,
            name,
            password,
            created_at: now,
            updated_at: now,
            last_used_at: now,
            last_changed_at: now,
        }
    }

    pub fn from_db(
        id: Uuid,
        name: String,
        password: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        last_used_at: DateTime<Utc>,
        last_changed_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            password,
            created_at,
            updated_at,
            last_used_at,
            last_changed_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn created_at(&self) -> String {
        let tmp = &self.created_at;
        tmp.to_rfc3339()
    }

    pub fn updated_at(&self) -> String {
        self.updated_at.to_rfc3339()
    }

    pub fn last_used_at(&self) -> String {
        self.last_used_at.to_rfc3339()
    }

    pub fn last_changed_at(&self) -> String {
        self.last_changed_at.to_rfc3339()
    }

    pub fn update_password(&mut self) {
        let now = Utc::now();
        let pg = PasswordGenerator::new()
            .length(32)
            .numbers(true)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(true)
            .spaces(false)
            .exclude_similar_characters(true)
            .strict(true);
        self.password = pg.generate_one().unwrap();
        self.updated_at = now;
        self.last_changed_at = now;
    }

    pub fn update_last_used_at(&mut self) {
        let now = Utc::now();
        self.last_used_at = now;
    }

    pub fn update_in_database(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open("keys.db")?;
        conn.execute(
            "UPDATE keys SET name = ?2, password = ?3, updated_at = ?4, last_used_at = ?5, last_changed_at = ?6 WHERE id = ?1",
            rusqlite::params![
                self.id,
                self.name,
                self.password,
                self.updated_at,
                self.last_used_at,
                self.last_changed_at
            ],
        )?;
        Ok(())
    }

    pub fn persist(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open("keys.db")?;
        conn.execute(
            "INSERT INTO keys (id, name, password, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                self.id,
                self.name,
                self.password,
                self.created_at,
                self.updated_at
            ],
        )?;
        Ok(())
    }

    pub fn retrive_keys_from_db() -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open("keys.db")?;
        let mut stmt = conn.prepare("SELECT id, name, password, created_at, updated_at, last_used_at, last_changed_at FROM keys" ).unwrap();
        let rows = stmt.query_map(rusqlite::params![], |row| {
            Ok(Key::from_db(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
            ))
        })?;

        let mut keys = Vec::new();
        for key in rows {
            keys.push(key?);
        }
        Ok(keys)
    }

    pub fn to_vec(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.name.clone(),
            self.password.clone(),
            self.created_at(),
            self.updated_at(),
            self.last_used_at(),
            self.last_changed_at(),
        ]
    }
}

// impl Iterator for Key {
//     type Item = Key;

//     fn next(&mut self) -> Option<Self::Item> {
//         debug!("Iterating over key");
//         Some(Key::new(
//             Some(self.id),
//             self.name.clone(),
//             self.value.clone(),
//             self.created_at,
//             self.updated_at,
//         ))
//     }
// }
