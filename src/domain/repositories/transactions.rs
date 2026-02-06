use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::transaction::TransactionEntity;

#[async_trait]
#[automock]
pub trait TransactionRepository {
    async fn deposit(&self,transaction : TransactionEntity) -> Result<()> ; 
    async fn withdraw(&self,transaction : TransactionEntity) -> Result<()> ; 
    async fn transfer(&self,transaction : TransactionEntity) -> Result<()> ; 
}