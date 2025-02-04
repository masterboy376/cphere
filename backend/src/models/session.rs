use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use mongodb::bson::{oid::ObjectId, doc, Document, DateTime as BsonDateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub user_id: ObjectId,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub fn new(user_id: ObjectId, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: None,
            user_id,
            expires_at,
        }
    }

    pub fn collection_name() -> &'static str {
        "sessions"
    }

    pub fn to_document(&self) -> Document {
        let mut doc = doc! {
            "user_id": &self.user_id,
            "expires_at": BsonDateTime::from_millis(self.expires_at.timestamp_millis()), // Convert `chrono::DateTime<Utc>` to `bson::DateTime`
        };

        if let Some(ref id) = self.id {
            doc.insert("_id", id);
        }

        doc
    }
}
