use chrono::prelude::*;
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime, Document};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub notification_type: String,
    pub recipient_id: ObjectId,
    pub sender_id: ObjectId,
    pub message: String,
    pub is_handled: bool,
    pub created_at: DateTime<Utc>,
}

impl Notification {
    pub fn new(
        notification_type: &str,
        recipient_id: ObjectId,
        sender_id: ObjectId,
        message: &str,
    ) -> Self {
        Self {
            id: None,
            notification_type: notification_type.to_owned(),
            recipient_id,
            sender_id,
            message: message.to_owned(),
            is_handled: false,
            created_at: Utc::now(),
        }
    }

    pub fn collection_name() -> &'static str {
        "notifications"
    }

    pub fn to_document(&self) -> Document {
        let mut doc = doc! {
            "notification_type": &self.notification_type,
            "recipient_id": &self.recipient_id,
            "sender_id": &self.sender_id,
            "message": &self.message,
            "is_handled": self.is_handled,
            "created_at": BsonDateTime::from_millis(self.created_at.timestamp_millis()), // Convert `chrono::DateTime<Utc>` to `bson::DateTime`
        };
        if let Some(ref id) = self.id {
            doc.insert("_id", id);
        }

        doc
    }
}
