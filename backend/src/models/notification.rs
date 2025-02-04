use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use mongodb::bson::{oid::ObjectId, doc, Document, DateTime as BsonDateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub user_id: ObjectId,
    pub message: String,
    pub read: bool,

    pub created_at: DateTime<Utc>,
}

impl Notification {
    pub fn new(user_id: ObjectId, message: &str) -> Self {
        Self {
            id: None,
            user_id,
            message: message.to_owned(),
            read: false,
            created_at: Utc::now(),
        }
    }

    pub fn collection_name() -> &'static str {
        "notifications"
    }

    pub fn to_document(&self) -> Document {
        let mut doc = doc! {
            "user_id": &self.user_id,
            "message": &self.message,
            "read": self.read,
            "created_at": BsonDateTime::from_millis(self.created_at.timestamp_millis()), // Convert `chrono::DateTime<Utc>` to `bson::DateTime`
        };

        if let Some(ref id) = self.id {
            doc.insert("_id", id);
        }

        doc
    }
}
