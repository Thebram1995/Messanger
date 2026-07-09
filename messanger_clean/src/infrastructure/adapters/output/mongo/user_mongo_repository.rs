use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};

use crate::application::ports::output::user_repository::UserRepository;
use crate::domain::entities::user::{User, UserLookup};

pub struct MongoUserRepository {
    collection: Collection<User>,
}

impl MongoUserRepository {
    pub fn new(collection: Collection<User>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn create(&self, user: User) -> Result<User, String> {
        self.collection
            .insert_one(user.clone(), None)
            .await
            .map_err(|error| format!("Error insertando usuario: {}", error))?;

        Ok(user)
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<UserLookup>, String> {
        let mut cursor = self
            .collection
            .aggregate(
                vec![
                    doc! { "$match": { "_id": id } },
                    doc! {
                        "$lookup": {
                            "from": "roles",
                            "localField": "role_id",
                            "foreignField": "_id",
                            "as": "role"
                        }
                    },
                    doc! { "$unwind": "$role" },
                ],
                None,
            )
            .await
            .map_err(|error| format!("Error buscando usuario por id: {}", error))?;

        if let Some(document) = cursor
            .try_next()
            .await
            .map_err(|error| format!("Error leyendo cursor: {}", error))?
        {
            let user = mongodb::bson::from_document::<UserLookup>(document)
                .map_err(|error| format!("Error parseando usuario: {}", error))?;

            return Ok(Some(user));
        }

        Ok(None)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<UserLookup>, String> {
        let mut cursor = self
            .collection
            .aggregate(
                vec![
                    doc! { "$match": { "username": username } },
                    doc! {
                        "$lookup": {
                            "from": "roles",
                            "localField": "role_id",
                            "foreignField": "_id",
                            "as": "role"
                        }
                    },
                    doc! { "$unwind": "$role" },
                ],
                None,
            )
            .await
            .map_err(|error| format!("Error buscando usuario por username: {}", error))?;

        if let Some(document) = cursor
            .try_next()
            .await
            .map_err(|error| format!("Error leyendo cursor: {}", error))?
        {
            let user = mongodb::bson::from_document::<UserLookup>(document)
                .map_err(|error| format!("Error parseando usuario: {}", error))?;

            return Ok(Some(user));
        }

        Ok(None)
    }

    async fn exists_by_username(&self, username: &str) -> Result<bool, String> {
        let count = self
            .collection
            .count_documents(doc! { "username": username }, None)
            .await
            .map_err(|error| format!("Error validando username: {}", error))?;

        Ok(count > 0)
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, String> {
        let count = self
            .collection
            .count_documents(doc! { "email": email }, None)
            .await
            .map_err(|error| format!("Error validando email: {}", error))?;

        Ok(count > 0)
    }

    async fn update(&self, user: User) -> Result<User, String> {
        self.collection
            .replace_one(doc! { "_id": user.id }, user.clone(), None)
            .await
            .map_err(|error| format!("Error actualizando usuario: {}", error))?;

        Ok(user)
    }

    async fn delete(&self, id: ObjectId) -> Result<(), String> {
        self.collection
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "active": false } },
                None,
            )
            .await
            .map_err(|error| format!("Error eliminando usuario: {}", error))?;

        Ok(())
    }
}