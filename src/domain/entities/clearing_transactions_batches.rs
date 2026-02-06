use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearingTransactionBatchEntity {
    pub clearing_item_id: i64,
    pub transaction_id: i64,
    pub account_id: i64,
    pub amount: f64,
    pub bank_code: String,
    pub job_id: i64,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
}
