use sqlx::{migrate::MigrateDatabase, Error as SqlxError, Sqlite, SqlitePool};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum DbManagerError {
    #[error("Failed to open the connection to the db: {0}")]
    OpenConnection(#[from] SqlxError),
}

#[allow(dead_code)]
pub struct DbManager {
    pub connection_pool: SqlitePool,
}

impl DbManager {
    #[allow(dead_code)]
    pub async fn new(db_url: &str) -> Result<DbManager, DbManagerError> {
        let pool = DbManager::db_setup(db_url).await?;

        Ok(DbManager {
            connection_pool: pool,
        })
    }

    #[allow(dead_code)]
    pub async fn db_setup(db_url: &str) -> Result<SqlitePool, DbManagerError> {
        let db;

        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            info!("Creating database: {db_url}");
            Sqlite::create_database(db_url).await?;

            db = SqlitePool::connect(db_url).await?;

            // Create the tables - move this to a function
            let _query_result = sqlx::query(
                "CREATE TABLE IF NOT EXISTS users \
                (username VARCHAR(250) NOT NULL UNIQUE, \
                password VARCHAR(250) NOT NULL UNIQUE);",
            )
            .execute(&db)
            .await?;

            let _query_result = sqlx::query(
                "CREATE TABLE IF NOT EXISTS karma \
                (id INTEGER PRIMARY KEY NOT NULL UNIQUE, \
                purpose_type INTEGER NOT NULL, \
                name VARCHAR(50) NOT NULL UNIQUE);",
            )
            .execute(&db)
            .await?;

            let _query_result = sqlx::query(
                "CREATE TABLE IF NOT EXISTS karma_status \
                (id INTEGER PRIMARY KEY NOT NULL UNIQUE, \
                karma_id INTEGER NOT NULL, \
                closed_with_purpose_type INTEGER NOT NULL, \
                current_state VARCHAR(50) NOT NULL, \
                timestamp BIGINT NOT NULL, \
                FOREIGN KEY(karma_id) REFERENCES karma(id));",
            )
            .execute(&db)
            .await?;
        } else {
            db = SqlitePool::connect(db_url).await?;
            info!("Database: {db_url} already exists");
        }

        // create the db connection pool

        Ok(db)
    }
}

impl AsRef<DbManager> for DbManager {
    fn as_ref(&self) -> &DbManager {
        self
    }
}

#[cfg(test)]
pub mod db_tests {
    use super::*;
    use crate::storage::common_utilities_tests::{initialize_db, DB};

    #[tokio::test]
    pub async fn test_db_manager() {
        initialize_db().await;
        assert!(std::path::Path::is_file(std::path::Path::new(
            "test_db.sqlite"
        )));
    }
}
