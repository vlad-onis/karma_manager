use thiserror::Error;

use crate::storage::db::{DbManager, DbManagerError};

#[derive(Error, Debug)]
pub enum AccountsServiceError {
    #[error("Failed to create the db manager: {0}")]
    DbManager(#[from] DbManagerError),
}

#[allow(dead_code)]
pub struct AccountsService {
    db_manager: DbManager,
}

#[allow(dead_code)]
impl AccountsService {
    pub async fn new(db_url: &str) -> Result<AccountsService, AccountsServiceError> {
        let db_manager = DbManager::new(db_url).await?;
        Ok(AccountsService { db_manager })
    }
}
