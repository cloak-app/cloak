use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Novel {
    pub id: i32,
    pub title: String,
    pub path: String,
    pub last_read_position: i64,
    pub total_lines: i64,
}
