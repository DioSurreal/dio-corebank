use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::fee::FeeEntity;

#[async_trait]
#[automock]
pub trait FeeRepository {
    async fn add_fee_policy(&self,fee:FeeEntity) -> Result<()> ; 
}