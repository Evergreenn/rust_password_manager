use chrono::prelude::*;
use passwords::PasswordGenerator;

#[derive(Debug, Clone)]
pub struct Password {
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_used_at: DateTime<Utc>,
    last_changed_at: DateTime<Utc>,
}

impl Password {
    pub fn new() -> Self {
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
}
