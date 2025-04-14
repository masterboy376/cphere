use chrono::prelude::*;
use mongodb::bson::{doc, oid::ObjectId, DateTime as BsonDateTime, Document};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_token_expiry_at: Option<i64>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: &str, email: &str, password_hash: &str) -> Self {
        Self {
            id: None,
            username: username.to_owned(),
            email: email.to_owned(),
            password_hash: password_hash.to_owned(),
            reset_token: None,
            reset_token_expiry_at: None,
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
        if let Some(ref reset_token) = self.reset_token {
            doc.insert("reset_token", reset_token);
        }
        if let Some(ref reset_token_expiry_at) = self.reset_token_expiry_at {
            doc.insert("reset_token_expiry_at", reset_token_expiry_at);
        }

        doc
    }
}
