use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::ledger_entries::LedgerEntryEntity;

#[async_trait]
#[automock]
pub trait LedgerEntryRepository {
    async fn booking_ledger(&self,ledger:LedgerEntryEntity) -> Result<()> ; 

}