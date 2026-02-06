use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEntity {
    pub fee_id: i64,
    pub fee_type: String,
    pub rate: f64,
    pub active: bool,
    pub updated_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}