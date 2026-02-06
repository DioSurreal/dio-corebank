use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearingType {
    ChequeClearing,
    InterbankClearing,
}


impl ToString for ClearingType {
     fn to_string(&self) -> String {
        match self {
            ClearingType::ChequeClearing => "Cheque_Clearing".to_string(),
            ClearingType::InterbankClearing => "Interbank_Clearing".to_string(),
           
        }
    }
}