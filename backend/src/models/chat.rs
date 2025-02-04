use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use mongodb::bson::{oid::ObjectId, doc, Document, DateTime as BsonDateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub participants: Vec<ObjectId>,

    pub created_at: DateTime<Utc>,
}

impl Chat {
    pub fn new(participants: Vec<ObjectId>) -> Self {
        Self {
            id: None,
            participants,
            created_at: Utc::now(),
        }
    }

    pub fn collection_name() -> &'static str {
        "chats"
    }

    pub fn to_document(&self) -> Document {
        let mut doc = doc! {
            "participants": &self.participants,
            "created_at": BsonDateTime::from_millis(self.created_at.timestamp_millis()), // Convert `chrono::DateTime<Utc>` to `bson::DateTime`
        };

        if let Some(ref id) = self.id {
            doc.insert("_id", id);
        }

        doc
    }
}
