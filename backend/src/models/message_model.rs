use chrono::prelude::*;
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub chat_id: ObjectId,
    pub sender_id: ObjectId,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Message {
    pub fn new(chat_id: ObjectId, sender_id: ObjectId, content: &str, created_at: Option<DateTime<Utc>>) -> Self {
        Self {
            id: None,
            chat_id,
            sender_id,
            content: content.to_owned(),
            created_at: match created_at {
                Some(dt) => dt,
                None => Utc::now(),
            }
        }
    }

    pub fn collection_name() -> &'static str {
        "messages"
    }

    pub fn to_document(&self) -> Document {
        let mut doc = doc! {
            "chat_id": &self.chat_id,
            "sender_id": &self.sender_id,
            "content": &self.content,
            "created_at": BsonDateTime::from_millis(self.created_at.timestamp_millis()), // Convert `chrono::DateTime<Utc>` to `bson::DateTime`
        };

        if let Some(ref id) = self.id {
            doc.insert("_id", id);
        }

        doc
    }
}
