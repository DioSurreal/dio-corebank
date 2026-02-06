use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::clearing_jobs::ClearingJobEntity;

#[async_trait]
#[automock]
pub trait ClearingJobRepository {
    async fn booking_clearing_job(&self,clearing_job: ClearingJobEntity) -> Result<()> ; 

}