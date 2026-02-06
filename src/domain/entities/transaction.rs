use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::money::Money;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEntity {
    pub idempotency_key: String,
    pub transaction_type: String,
    pub channel: String,
    pub amount: Money,
    pub receiver_bank_code: String,
    pub from_account_id: Option<i64>,
    pub to_account_id: Option<i64>,
    pub status: String,

}