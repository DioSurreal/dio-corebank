use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Money {
    amount: Decimal,
}

impl Money {
    pub fn new(amount: Decimal) -> Result<Self, String> {
        if amount < Decimal::from(0) {
            return Err("Money cannot be negative".into());
        }
        Ok(Self { amount })
    }

    pub fn amount(&self) -> &Decimal {
        &self.amount
    }

    pub fn deposit(&self, amount: Decimal) -> Money {
        Money {
            amount: self.amount + amount,
        }
    }

    pub fn withdraw(&self, amount: Decimal) -> Result<Money, MoneyError> {
        if self.amount <= amount {
            return Err(MoneyError::Insufficient);
        }
        Ok(Money {
            amount: self.amount - amount,
        })
    }
}
#[derive(Debug)]
pub enum MoneyError {
    Negative,
    Insufficient,
}