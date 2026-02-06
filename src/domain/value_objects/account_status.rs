use serde::{Deserialize, Serialize};




#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Frozen,
    Closed,
}

impl AccountStatus {
    pub fn can_transact(&self) -> bool {
        matches!(self, AccountStatus::Active)
    }
}
