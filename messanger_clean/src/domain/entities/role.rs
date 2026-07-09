use bson::DateTime as BsonDateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct Role {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub permissions: Vec<String>,
    pub active: bool,
    pub created_at: BsonDateTime,
    pub updated_at: BsonDateTime,
}