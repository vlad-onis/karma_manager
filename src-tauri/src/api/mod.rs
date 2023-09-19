pub mod karma_api;

use crate::service::karma::karma_service::KarmaService;
use crate::storage::db::{DbManager, DbManagerError};

use once_cell::sync::OnceCell;
use serde::Serialize;
use thiserror::Error;
use tokio::sync::Mutex;

static API_CONTROLLER: OnceCell<ApiController> = OnceCell::new();
static API_CONTROLLER_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

pub async fn get_controller() -> Result<&'static ApiController, ApiControllerError> {
    // this is racy, but that's OK: it's just a fast case
    if let Some(controller) = API_CONTROLLER.get() {
        return Ok(controller);
    }

    // it hasn't been initialized yet, so let's grab the lock & try to initialize it
    let initializing_mutex = API_CONTROLLER_INITIALIZED.get_or_init(|| Mutex::new(false));

    // this will wait if another task is currently initializing the client
    let mut initialized = initializing_mutex.lock().await;

    // if initialized is true, then someone else initialized it while we waited,
    // and we can just skip this part.
    if !*initialized {
        // no one else has initialized it yet, so
        let controller = ApiController::new().await?;

        API_CONTROLLER.set(controller).expect(
            "no one else should be initializing this \
            as we hold API_CONTROLLER_INITIALIZED lock",
        );
        *initialized = true;
        drop(initialized);
    }

    Ok(API_CONTROLLER.get().unwrap())
}

#[derive(Debug, Error, Serialize)]
pub enum ApiControllerError {
    #[error("Failed to initialise and conncet to the database")]
    DatabaseConnectionFailure(#[from] DbManagerError),
}

#[derive(Debug)]
pub struct ApiController {
    karma_service: KarmaService<DbManager>,
}

impl ApiController {
    pub async fn new() -> Result<ApiController, ApiControllerError> {
        let karma_repo = DbManager::new("karma_db.sqlite").await?;
        let karma_service = KarmaService::new(karma_repo);

        Ok(ApiController { karma_service })
    }
}
