/// Currently everything related to accounts is dropped as login and auth is a bit beyond
/// the scope of this app
use async_trait::async_trait;

use crate::model::user::User;
use crate::storage::db::{DbManager, DbManagerError};

#[async_trait]
pub trait UserRepository {
    async fn insert_user(&self, user: User) -> Result<User, DbManagerError>;
    async fn get_user(&self, username: &str) -> Result<User, DbManagerError>;
}

#[async_trait]
impl UserRepository for DbManager {
    async fn insert_user(&self, user: User) -> Result<User, DbManagerError> {
        let _query_result = sqlx::query("INSERT INTO users(username, password) VALUES(?, ?);")
            .bind(user.username.get_username())
            .bind(user.hashed_password.get_password())
            .execute(&self.connection_pool)
            .await?;

        Ok(user)
    }

    async fn get_user(&self, username: &str) -> Result<User, DbManagerError> {
        let query_result = sqlx::query_as::<_, User>("SELECT * FROM users where username=?")
            .bind(username)
            .fetch_one(&self.connection_pool)
            .await?;

        Ok(query_result)
    }
}
