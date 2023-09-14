use async_trait::async_trait;

use crate::model::karma::{KarmaPoint, KarmaStatus};
use crate::storage::db::{DbManager, DbManagerError};

#[async_trait]
pub trait KarmaRepository {
    async fn insert_karma(&self, karma: KarmaPoint) -> Result<KarmaPoint, DbManagerError>;
    async fn get_karma_by_name(&self, name: String) -> Result<KarmaPoint, DbManagerError>;
    async fn insert_karma_status(&self, status: KarmaStatus)
        -> Result<KarmaStatus, DbManagerError>;
    async fn get_karma_status(
        &self,
        karma_point: KarmaPoint,
    ) -> Result<KarmaStatus, DbManagerError>;
}

#[async_trait]
impl KarmaRepository for DbManager {
    async fn insert_karma(&self, karma: KarmaPoint) -> Result<KarmaPoint, DbManagerError> {
        let _ = sqlx::query("INSERT INTO karma(purpose, name) VALUES(?, ?);")
            .bind(karma.get_purpose() as i32)
            .bind(karma.get_name())
            .execute(&self.connection_pool)
            .await?;

        Ok(karma)
    }

    async fn get_karma_by_name(&self, name: String) -> Result<KarmaPoint, DbManagerError> {
        let karma_point_result =
            sqlx::query_as::<_, KarmaPoint>("SELECT * FROM karma WHERE name = ?;")
                .bind(name)
                .fetch_one(&self.connection_pool)
                .await?;

        Ok(karma_point_result)
    }

    async fn insert_karma_status(
        &self,
        status: KarmaStatus,
    ) -> Result<KarmaStatus, DbManagerError> {
        sqlx::query::<sqlx::Sqlite>(
            "INSERT INTO karma_status(karma_id, current_state, timestamp) VALUES(?, ?, ?);",
        )
        .bind(status.karma_id)
        .bind(status.state.to_string())
        .bind(status.timestamp)
        .execute(&self.connection_pool)
        .await?;

        Ok(status)
    }

    async fn get_karma_status(
        &self,
        karma_point: KarmaPoint,
    ) -> Result<KarmaStatus, DbManagerError> {
        let karma_point = self.get_karma_by_name(karma_point.get_name()).await?;

        // shouldn't fail
        let karma_id = karma_point.get_id().unwrap();

        let karma_status_result =
            sqlx::query_as::<_, KarmaStatus>("SELECT * FROM karma_status WHERE karma_id = ?;")
                .bind(karma_id)
                .fetch_one(&self.connection_pool)
                .await?;

        Ok(karma_status_result)
    }
}

#[cfg(test)]
pub mod karma_repository_tests {
    use super::*;
    use crate::{
        model::karma::{KarmaType, State},
        storage::common_utilities_tests::{setup_once, DB},
    };

    async fn get_or_insert(karma: KarmaPoint) -> KarmaPoint {
        setup_once().await;

        let retrieved_karma = DB
            .lock()
            .await
            .as_ref()
            .unwrap()
            .get_karma_by_name(karma.get_name())
            .await;

        if retrieved_karma.is_err() {
            let inserted_karma = DB
                .lock()
                .await
                .as_ref()
                .unwrap()
                .insert_karma(karma.clone())
                .await
                .expect("Failed to insert the karma point");

            // Get it again from the DB for the ID. This should not fail
            let retrieved_karma = DB
                .lock()
                .await
                .as_ref()
                .unwrap()
                .get_karma_by_name(inserted_karma.get_name())
                .await
                .unwrap();
            return retrieved_karma;
        } else {
            return retrieved_karma.unwrap();
        }
    }

    #[tokio::test]
    async fn test_karma_point_operations() {
        let karma = KarmaPoint::new(KarmaType::Sport, "Sporty karma".to_string());
        setup_once().await;
        let name = karma.get_name();
        let inserted_karma = DB
            .lock()
            .await
            .as_ref()
            .unwrap()
            .insert_karma(karma)
            .await
            .expect("Failed to insert the karma point");

        assert_eq!(name, inserted_karma.get_name());

        let _retrieved_karma = DB
            .lock()
            .await
            .as_ref()
            .unwrap()
            .get_karma_by_name("Sporty karma".to_string())
            .await
            .expect("Could not retrieve karma point by name");
    }

    #[tokio::test]
    async fn test_karma_status_operations() {
        let karma = KarmaPoint::new(KarmaType::Learning, "Learning K".to_string());
        let karma = get_or_insert(karma).await;

        if let Some(id) = karma.get_id() {
            let timestamp = chrono::Utc::now().timestamp();
            let karma_status = KarmaStatus::new(id, State::Active, timestamp);

            let _inserted_karma_status = DB
                .lock()
                .await
                .as_ref()
                .unwrap()
                .insert_karma_status(karma_status)
                .await
                .expect("Failed to insert the karma point");
        } else {
            assert!(false);
        }

        let karma_status = DB
            .lock()
            .await
            .as_ref()
            .unwrap()
            .get_karma_status(karma.clone())
            .await
            .expect("Failed to retrieve the karma status");

        assert_eq!(karma_status.karma_id, karma.get_id().unwrap());
    }
}
