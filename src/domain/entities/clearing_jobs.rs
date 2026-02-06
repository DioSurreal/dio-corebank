use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::clearing_type::ClearingType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearingJobEntity {
    pub job_id: i64,
    pub clearing_type: ClearingType,
    pub status: String,
    pub created_at: NaiveDateTime,
}