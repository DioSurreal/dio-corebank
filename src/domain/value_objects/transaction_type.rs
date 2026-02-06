use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAW,
    TRANSFER
}


impl ToString for TransactionType {
     fn to_string(&self) -> String {
        match self {
            TransactionType::DEPOSIT => "DEPOSIT".to_string(),
            TransactionType::WITHDRAW => "WITHDRAW".to_string(),
            TransactionType::TRANSFER =>"TRANSFER".to_string()
        }
    }
}