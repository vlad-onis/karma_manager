use async_trait::async_trait;

use crate::model::karma::KarmaPoint;
use crate::storage::db::{DbManager, DbManagerError};

#[async_trait]
pub trait KarmaRepository {
    async fn insert_karma(&self, karma: KarmaPoint) -> Result<KarmaPoint, DbManagerError>;
    async fn get_karma_by_name(&self, name: String) -> Result<KarmaPoint, DbManagerError>;
}

#[async_trait]
impl KarmaRepository for DbManager {
    async fn insert_karma(&self, karma: KarmaPoint) -> Result<KarmaPoint, DbManagerError> {
        let _query_result = sqlx::query("INSERT INTO karma(purpose, name) VALUES(?, ?);")
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
}

#[cfg(test)]
pub mod karma_repository_tests {
    use super::*;
    use crate::{
        model::karma::KarmaType,
        storage::common_utilities_tests::{initialize_db, DB},
    };

    #[tokio::test]
    async fn test_insertion() {
        let karma = KarmaPoint::new(KarmaType::Sport, "Sporty karma".to_string());

        initialize_db().await;
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
}
