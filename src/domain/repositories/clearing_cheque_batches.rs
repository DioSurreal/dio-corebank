use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::clearing_cheque_batches::ClearingChequeBatchEntity;

#[async_trait]
#[automock]
pub trait ClearingchequeBatchRepository {
    async fn clearing_batch_cheque(&self,clearing_cheque_batch : ClearingChequeBatchEntity) -> Result<()> ; 

}