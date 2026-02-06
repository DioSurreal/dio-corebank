use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChequeStatus {
    Received,
    PendingClearing,
    Cleared,
    Returned,
    Settled,
    Cancelled,
}

impl ToString for ChequeStatus {
    fn to_string(&self) -> String {
        match self {
            ChequeStatus::Received => "RECEIVED".to_string(),
            ChequeStatus::PendingClearing => "PENDING_CLEARING".to_string(),
            ChequeStatus::Cleared => "CLEARED".to_string(),
            ChequeStatus::Returned => "RETURNED".to_string(),
            ChequeStatus::Settled => "SETTLED".to_string(),
            ChequeStatus::Cancelled => "CANCELLED".to_string(),
        }
    }
}