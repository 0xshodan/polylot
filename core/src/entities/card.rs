use super::definition::Definition;
use anyhow::{bail, Result};
use chrono::{DateTime, Utc};

pub struct Card {
    pub name: String,
    pub definitions: Vec<Definition>,
}

impl Card {
    pub fn new(self, name: String, definitions: Vec<Definition>) -> Result<Self> {
        if definitions.is_empty() {
            bail!("Minimum one definiton must be specified");
        }
        Ok(Card {
            name: name,
            definitions: definitions,
        })
    }
}
