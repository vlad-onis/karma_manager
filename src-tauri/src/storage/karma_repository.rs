use async_trait::async_trait;

use crate::model::karma::KarmaPoint;
use crate::storage::db::{DbManager, DbManagerError};

#[async_trait]
pub trait KarmaRepository {
    async fn insert_karma(&self, karma: KarmaPoint) -> Result<KarmaPoint, DbManagerError>;
}

#[async_trait]
impl KarmaRepository for DbManager {
    async fn insert_karma(&self, karma: KarmaPoint) -> Result<KarmaPoint, DbManagerError> {
        let _query_result = sqlx::query("INSERT INTO karma(purpose_type, name) VALUES(?, ?);")
            .bind(karma.get_purpose() as i32)
            .bind(karma.get_name())
            .execute(&self.connection_pool)
            .await?;

        Ok(karma)
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
    }
}
