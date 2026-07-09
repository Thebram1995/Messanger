use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

use crate::domain::entities::role::Role;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, role: Role) -> Result<Role, String>;

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Role>, String>;

    async fn find_all(&self) -> Result<Vec<Role>, String>;

    async fn exists_by_name(&self, name: &str) -> Result<bool, String>;

    async fn find_by_name(&self, name: &str) -> Result<Option<Role>, String>;
}