use crate::model::karma::KarmaPoint;
use crate::storage::db::DbManagerError;
use crate::storage::karma_repository::KarmaRepository;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum KarmaServiceError {
    #[error("Storage failed with: {0}")]
    Storage(#[from] DbManagerError),
}

#[derive(Debug)]
pub struct KarmaService<R: KarmaRepository> {
    karma_repository: R,
}

impl<R: KarmaRepository> KarmaService<R> {
    pub fn new(karma_repository: R) -> Self {
        KarmaService { karma_repository }
    }

    pub async fn create_karma(&self, karma: KarmaPoint) -> Result<KarmaPoint, KarmaServiceError> {
        self.karma_repository
            .insert_karma(karma)
            .await
            .map_err(|e| e.into())
    }
}
