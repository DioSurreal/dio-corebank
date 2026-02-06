use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::value_objects::{
    account_status::AccountStatus,
    money::{Money, MoneyError},
};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq)]
#[diesel(table_name = accounts)]
pub struct AccountEntity {
    pub account_id: i64,
    pub customer_id: i64,
    pub account_number: String,
    pub balance: Money, // Note: In a real production system, use BigDecimal
    pub status: AccountStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, PartialEq)]
#[diesel(table_name = accounts)]
pub struct AddAccountEntity {
    pub customer_id: i64,
    pub account_number: String,
    pub balance: Money, // Note: In a real production system, use BigDecimal
    pub status: AccountStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("account is not active")]
    AccountNotActive,

    #[error("insufficient balance")]
    InsufficientBalance,
}

impl From<MoneyError> for AccountError {
    fn from(err: MoneyError) -> Self {
        match err {
            MoneyError::Insufficient => AccountError::InsufficientBalance,
            MoneyError::Negative => AccountError::InsufficientBalance,
        }
    }
}

//ไปอยู่ตรงapplicationแทนเหอะ
// #[derive(Debug, Clone, Serialize, Deserialize ,Queryable)]
// #[diesel(table_name = accounts)]
// pub struct CheckAccountsExist {
//     pub account_id: i64,
//     pub balance: Balance, // Note: In a real production system, use BigDecimal
//     pub status: AccountStatus,
// }

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = accounts)]
pub struct UpdateAccount {
    pub balance: Money,
    pub updated_at: NaiveDateTime,
    pub status: AccountStatus,
}

impl UpdateAccount {
    pub fn deposit(&mut self, amount: Decimal) -> Result<(), AccountError> {
        if !self.status.can_transact() {
            return Err(AccountError::AccountNotActive);
        }

        self.balance = self.balance.deposit(amount);
        Ok(())
    }

    pub fn withdraw(&mut self, amount: Decimal) -> Result<(), AccountError> {
        if !self.status.can_transact() {
            return Err(AccountError::AccountNotActive);
        }

        self.balance = self.balance.withdraw(amount)?;
        Ok(())
    }
}
