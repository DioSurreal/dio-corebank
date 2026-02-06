use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::cheque_transaction::ChequeTransactionEntity;

#[async_trait]
#[automock]
pub trait ChequeTransactionRepository {
    async fn cheque_transaction(&self,cheque:ChequeTransactionEntity) -> Result<()> ; 

}