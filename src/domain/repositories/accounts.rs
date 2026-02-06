use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::{account::{AccountEntity, UpdateAccount}, cheque_transaction::ChequeTransactionEntity};

#[async_trait]
#[automock]
pub trait AccountRepository {
    async fn check_account_exists(&self,account_number:String) -> AccountEntity ; 
    async fn update_account(&self,account_update: UpdateAccount) -> Result<()> ; 
    async fn add_account(&self,account : AccountEntity) -> Result<()> ; 
}