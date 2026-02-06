use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::customer::CustomerEntity;

#[async_trait]
#[automock]
pub trait TransactionRepository {
    async fn add_customer(&self,customer:CustomerEntity) -> Result<()> ; 

}