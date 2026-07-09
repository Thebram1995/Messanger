use bson::DateTime as BsonDateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::domain::entities::role::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub role_id: ObjectId,
    pub active: bool,
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
    pub created_by: Option<ObjectId>,
    pub updated_by: Option<ObjectId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLookup {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub role_id: ObjectId,
    pub active: bool,
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
    pub created_by: Option<ObjectId>,
    pub updated_by: Option<ObjectId>,
    pub role: Role,
}
