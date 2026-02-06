use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAccountEntity {
    pub system_account_id: i64,
    pub code: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}