use super::card::Card;
use chrono::{DateTime, Utc};
pub struct Module {
    pub name: String,
    pub description: Optional<String>,
    pub cards: Vec<Card>,
    pub created_at: DateTime<Utc>,
}
