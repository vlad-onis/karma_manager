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

    // Static variable to track if setup has been done
    lazy_static! {
        static ref CLEAN_DB: Mutex<bool> = Mutex::new(false);
    }

    // Removes the db file before any test
    pub async fn setup_once() {
        let mut done = CLEAN_DB.lock().await;

        if !*done {
            let test_db_path = Path::new("test_db.sqlite");
            let test_db_meta1_path = Path::new("test_db.sqlite-shm");
            let test_db_meta2_path = Path::new("test_db.sqlite-wal");

            if Path::is_file(test_db_path) {
                let _ = std::fs::remove_file(test_db_path);
                let _ = std::fs::remove_file(test_db_meta1_path);
                let _ = std::fs::remove_file(test_db_meta2_path);

                println!("Removed test db files");
            }

            // Set the flag to true to indicate db cleanup is done
            *done = true;
        }

        let db_name = "test_db.sqlite";

        let mut db = DB.lock().await;
        if db.is_none() {
            // todo: Should we remove the db file everytime here?
            *db = Some(
                DbManager::new(db_name)
                    .await
                    .expect("Failed to create the test db manager"),
            );
            println!("Inited the db");
        } else {
            println!("The db was not none");
        }
    }
}
