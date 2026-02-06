use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{money::Money, cheque_status::ChequeStatus};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChequeTransactionEntity {
    pub cheque_id: i64,
    pub cheque_number: String,
    pub issuing_bank_code: String,
    pub issuing_account_number: String,
    pub amount: Money,
    pub depositor_account_id: i64,
    pub status: ChequeStatus,
    pub received_at: NaiveDateTime,
    pub cleared_at: Option<NaiveDateTime>,
    pub settled_at: Option<NaiveDateTime>,
}