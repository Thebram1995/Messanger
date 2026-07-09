use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::domain::entities::user::{User, UserLookup};

#[async_trait]
pub trait UserRepository: Send + Sync {

    async fn create(
        &self,
        user: User,
    ) -> Result<User, String>;

    async fn find_by_id(
        &self,
        id: ObjectId,
    ) -> Result<Option<UserLookup>, String>;

    async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<UserLookup>, String>;

    async fn exists_by_username(
        &self,
        username: &str,
    ) -> Result<bool, String>;

    async fn update(
        &self,
        user: User,
    ) -> Result<User, String>;

    async fn delete(
        &self,
        id: ObjectId,
    ) -> Result<(), String>;

    async fn exists_by_email(
        &self, 
        email: &str
    ) -> Result<bool, String>;
}