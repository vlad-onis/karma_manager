pub mod db;
pub mod karma_repository;
pub mod user_repository;

#[cfg(test)]
pub mod common_utilities_tests {
    use std::sync::Arc;

    use crate::storage::db::DbManager;
    use lazy_static::lazy_static;
    use std::path::Path;
    use tokio::sync::Mutex;

    lazy_static! {
        pub static ref DB: Arc<Mutex<Option<DbManager>>> = Arc::new(Mutex::new(None));
    }

    pub async fn initialize_db() {
        let db_name = "test_db.sqlite";

        let mut db = DB.lock().await;
        if db.is_none() {
            if Path::is_file(Path::new(db_name)) {
                std::fs::remove_file(Path::new(db_name))
                    .expect("Could not delete existing test db");
            }
            *db = Some(
                DbManager::new("test_db.sqlite")
                    .await
                    .expect("Failed to create the test db"),
            );
        }
    }
}
