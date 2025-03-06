use chrono::prelude::*;
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub participant_ids: Vec<ObjectId>,
    pub created_at: DateTime<Utc>,
}

impl Chat {
    pub fn new(
        id: Option<ObjectId>,
        participant_ids: Vec<ObjectId>,
        created_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            participant_ids,
            created_at: created_at.unwrap_or_else(Utc::now),
        }
    }

    pub fn collection_name() -> &'static str {
        "chats"
    }

    pub fn to_document(&self) -> Document {
        let mut doc = doc! {
            "participant_ids": &self.participant_ids,
            "created_at": BsonDateTime::from_millis(
                self.created_at.timestamp_millis()), // Convert `chrono::DateTime<Utc>` to `bson::DateTime`
        };
        if let Some(ref id) = self.id {
            doc.insert("_id", id);
        }

        doc
    }
}
