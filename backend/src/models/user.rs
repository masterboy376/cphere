use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::prelude::*;
use mongodb::bson::{
    DateTime as BsonDateTime,
    doc,
    Document,
    oid::ObjectId,
};


#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: &str, email: &str, password_hash: &str) -> Self {
        Self {
            id: None,
            username: username.to_owned(),
            email: email.to_owned(),
            password_hash: password_hash.to_owned(),
            created_at: Utc::now(),
        }
    }

    pub fn collection_name() -> &'static str {
        "users"
    }

    pub fn to_document(&self) -> Document {
        let mut doc = doc! {
            "username": &self.username,
            "email": &self.email,
            "password_hash": &self.password_hash,
            "created_at": BsonDateTime::from_millis(self.created_at.timestamp_millis()), // Convert `chrono::DateTime<Utc>` to `bson::DateTime`
        };
        if let Some(ref id) = self.id {
            doc.insert("_id", id);
        }

        doc
    }
}
