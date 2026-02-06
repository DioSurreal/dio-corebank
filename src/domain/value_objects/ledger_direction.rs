use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LedgerDirection {
    Debit,
    Credit,
}

impl ToString for LedgerDirection {
    fn to_string(&self) -> String {
        match self {
            LedgerDirection::Debit => "DEBIT".to_string(),
            LedgerDirection::Credit => "CREDIT".to_string(),
        }
    }
}