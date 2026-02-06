use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::clearing_transactions_batches::ClearingTransactionBatchEntity;

#[async_trait]
#[automock]
pub trait ClearingTransactionBatchRepository {
    async fn clearing_batch_transaction(&self,clearing_transaction_batch:ClearingTransactionBatchEntity) -> Result<()> ; 

}