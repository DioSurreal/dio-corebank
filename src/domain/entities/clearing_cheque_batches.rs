use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{money::Money, cheque_status::ChequeStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearingChequeBatchEntity {
    pub clearing_cheque_id: i64,
    pub cheque_id: i64,
    pub account_id: i64,
    pub amount: Money,
    pub bank_code: String,
    pub job_id: i64,
    pub status: ChequeStatus,
    pub created_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
}