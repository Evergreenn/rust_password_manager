#[derive(Debug, Clone)]
pub struct Key {
    id: i64,
    name: String,
    value: String,
    created_at: String,
    updated_at: String,
}

impl Key {
    pub fn new(
        id: i64,
        name: String,
        value: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
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

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}
