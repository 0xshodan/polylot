use super::super::module::Module;
use chrono::{DateTime, Utc};

pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub modules: Vec<Module>,
    pub created_at: DateTime<Utc>,
}
