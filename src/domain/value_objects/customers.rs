use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBankAccount {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Customer {
    pub name: String,
    pub status: CustomerStatus,
    pub created_at: NaiveDateTime,
}

impl Customer {
    pub fn new_for_account_opening(cmd: CreateBankAccount) -> Self {
        Self {
            name: cmd.name,
            status: CustomerStatus::Active,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CustomerStatus {
    Active,
    Limited,
    Closed,
}

impl CustomerStatus {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            CustomerStatus::Active => "ACTIVE",
            CustomerStatus::Limited => "LIMITED",
            CustomerStatus::Closed => "CLOSED",
        }
    }

    pub fn from_db(value: &str) -> anyhow::Result<Self> {
        match value {
            "ACTIVE" => Ok(Self::Active),
            "LIMITED" => Ok(Self::Limited),
            "CLOSED" => Ok(Self::Closed),
            _ => Err(anyhow::anyhow!("Invalid status")),
        }
    }
}
