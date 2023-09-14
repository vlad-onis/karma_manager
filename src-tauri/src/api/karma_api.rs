use thiserror::Error;

use crate::{
    service::karma::karma_service::{KarmaService, KarmaServiceError},
    storage::{
        db::{DbManager, DbManagerError},
        karma_repository::KarmaRepository,
    },
};
use serde::Serialize;
use tracing::info;

#[derive(Error, Debug, Serialize)]
pub enum KarmaApiError {
    #[error("Failed to convert karma type: {0}")]
    InvalidKarmaType(String),

    #[error("Failed to create the Db manager")]
    DbCreationError(#[from] DbManagerError),

    #[error("Failed to create karma point: {0}")]
    KarmaCreationFailed(#[from] KarmaServiceError),
}

pub struct KarmaApiController {
    karma_service: KarmaService<DbManager>,
}

impl KarmaApiController {
    pub async fn new() -> Result<KarmaApiController, KarmaApiError> {
        let karma_repo = DbManager::new("karma_db.sqlite").await?;
        let karma_service = KarmaService::new(karma_repo);

        Ok(KarmaApiController { karma_service })
    }
}

pub mod create {
    use super::{KarmaApiController, KarmaApiError};
    use crate::{
        model::karma::{KarmaPoint, KarmaType},
        storage::karma_repository::KarmaRepository,
    };
    use tracing::info;

    impl TryFrom<&str> for KarmaType {
        type Error = KarmaApiError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            // Todo: handle all the other arms
            match value {
                "work" => Ok(KarmaType::Work),
                _ => Err(KarmaApiError::InvalidKarmaType(value.to_string())),
            }
        }
    }

    #[tauri::command]
    pub async fn create_karma(name: String, purpose: String) -> Result<(), KarmaApiError> {
        let karma_type = KarmaType::try_from(purpose.as_str())?;
        let karma_point = KarmaPoint::new(karma_type, name);

        // Todo: do not create the controller here
        let karma_controller = KarmaApiController::new().await?;

        let inserted_karma_point = karma_controller
            .karma_service
            .create_karma(karma_point)
            .await?;
        info!("Created: {inserted_karma_point:?}");
        Ok(())
    }

    pub fn handle_create() {}
}
