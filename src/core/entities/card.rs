use super::definition::Definition;
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};

pub struct Card {
    pub name: String,
    pub definitions: Vec<Definition>,
    pub created_at: DateTime<Utc>,
}

impl Card {
    pub fn new(
        self,
        name: String,
        definitions: Vec<Definition>,
        created_at: DateTime<Utc>,
    ) -> Result<Self> {
        if definitions.is_empty() {
            bail!("Minimum one definiton must be specified");
        }
        Ok(Card {
            name: name,
            definitions: definitions,
            created_at: created_at,
        })
    }
}
