use std::fmt::{Display, Formatter};

use chrono::prelude::*;
use passwords::PasswordGenerator;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Password {
    id: Uuid,
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_used_at: DateTime<Utc>,
    last_changed_at: DateTime<Utc>,
}

impl Password {
    pub fn new() -> Self {
        let now = Utc::now();
        let id = Uuid::new_v4();
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
            password,
            created_at: now,
            updated_at: now,
            last_used_at: now,
            last_changed_at: now,
        }
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn last_used_at(&self) -> String {
        self.last_used_at.to_rfc3339()
    }

    pub fn last_changed_at(&self) -> String {
        self.last_changed_at.to_rfc3339()
    }

    pub fn updated_at(&self) -> String {
        self.updated_at.to_rfc3339()
    }

    pub fn created_at(&self) -> String {
        self.created_at.to_rfc3339()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn persist(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open("keys.db")?;

        conn.execute(
            "INSERT INTO passwords (id, password, created_at, updated_at, last_used_at, last_changed_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![self.id, self.password, self.created_at, self.updated_at, self.last_used_at, self.last_changed_at],
        )?;

        Ok(())
    }

    pub fn get_password_by_id(id: Uuid) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open("keys.db")?;

        let mut stmt = conn.prepare("SELECT * FROM passwords WHERE id = ?1")?;
        let rows = stmt.query_map(rusqlite::params![id], |rows| {
            Ok(Self {
                id: rows.get(0)?,
                password: rows.get(1)?,
                created_at: rows.get(2)?,
                updated_at: rows.get(3)?,
                last_used_at: rows.get(4)?,
                last_changed_at: rows.get(5)?,
            })
        })?;

        let mut passwords = Vec::new();

        for row in rows {
            passwords.push(row?);
        }
        Ok(passwords[0].clone())
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.password)
    }
}
