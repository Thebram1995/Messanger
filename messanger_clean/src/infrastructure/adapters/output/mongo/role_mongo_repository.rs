use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

use crate::{
    application::ports::output::role_repository::RoleRepository,
    domain::entities::role::Role,
};

pub struct MongoRoleRepository {
    collection: Collection<Role>,
}

impl MongoRoleRepository {
    pub fn new(collection: Collection<Role>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl RoleRepository for MongoRoleRepository {
    async fn create(&self, role: Role) -> Result<Role, String> {
        self.collection
            .insert_one(role.clone(), None)
            .await
            .map_err(|error| format!("Error insertando rol: {}", error))?;

        Ok(role)
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Role>, String> {
        self.collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(|error| format!("Error buscando rol: {}", error))
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Role>, String> {
        self.collection
            .find_one(doc! { "name": name }, None)
            .await
            .map_err(|error| format!("Error buscando nombre: {}", error))
    }

    async fn find_all(&self) -> Result<Vec<Role>, String> {
        let mut cursor = self.collection
            .find(doc! { "active": true }, None)
            .await
            .map_err(|error| format!("Error listando roles: {}", error))?;

        let mut roles = Vec::new();

        while let Some(role) = cursor
            .try_next()
            .await
            .map_err(|error| format!("Error leyendo roles: {}", error))?
        {
            roles.push(role);
        }

        Ok(roles)
    }

    async fn exists_by_name(&self, name: &str) -> Result<bool, String> {
        let count = self.collection
            .count_documents(doc! { "name": name }, None)
            .await
            .map_err(|error| format!("Error validando rol: {}", error))?;

        Ok(count > 0)
    }
}