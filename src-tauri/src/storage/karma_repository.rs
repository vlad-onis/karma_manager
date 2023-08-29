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
        let _query_result =
            sqlx::query("INSERT INTO karma(purpose_type, close_type, closed) VALUES(?, ?, ?);")
                .bind(karma.get_purpose() as i32)
                .bind(karma.get_close_type() as i32)
                .bind(karma.get_closed_status())
                .execute(&self.connection_pool)
                .await?;

        Ok(karma)
    }
}
