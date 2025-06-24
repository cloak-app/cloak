use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Novel {
    pub id: i64,
    pub title: String,
    pub path: String,
    pub read_position: i64,
    pub read_progress: f64,
    pub file_size: i64,
    pub created_at: String,
    pub updated_at: String,
}
