use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{ledger_direction::LedgerDirection, money::Money};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntryEntity {
    pub transaction_id: i64,
    pub account_id: i64,
    pub direction: LedgerDirection,
    pub amount: Money,
    pub created_at: NaiveDateTime,
}