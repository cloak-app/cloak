use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Novel {
    pub id: i32,
    pub title: String,
    pub path: String,
    pub last_read_position: u64,
    pub total_characters: u64,
}
