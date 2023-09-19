use thiserror::Error;

use crate::{service::karma::karma_service::KarmaServiceError, storage::db::DbManagerError};
use serde::Serialize;

use super::ApiControllerError;

#[derive(Error, Debug, Serialize)]
pub enum KarmaApiError {
    #[error("Failed to convert karma type: {0}")]
    InvalidKarmaType(String),

    #[error("Failed to initialise the api controller because: {0}")]
    ApiControllerFailure(#[from] ApiControllerError),

    #[error("Failed to create karma point: {0}")]
    KarmaCreationFailed(#[from] KarmaServiceError),
}

pub mod create {
    use super::KarmaApiError;
    use crate::api::get_controller;
    use crate::model::karma::{KarmaPoint, KarmaType};

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

        let controller = get_controller().await?;

        let inserted_karma_point = controller.karma_service.create_karma(karma_point).await?;
        info!("Created: {inserted_karma_point:?}");
        Ok(())
    }
}
