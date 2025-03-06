use cphere_backend::models::message_model::Message;
use chrono::Utc;
use mongodb::bson::oid::ObjectId;

#[test]
fn test_create_message() {
    // Create dummy IDs for chat and sender.
    let chat_id = ObjectId::new();
    let sender_id = ObjectId::new();

    // Create a message using the correct parameters.
    let message = Message::new(chat_id, sender_id, "Hello, world!");

    // Check that the fields are set as expected.
    assert_eq!(message.chat_id, chat_id);
    assert_eq!(message.sender_id, sender_id);
    assert_eq!(message.content, "Hello, world!");
    // Verify that the created_at timestamp is not in the future.
    assert!(message.created_at <= Utc::now());
}

#[test]
fn test_empty_message_content() {
    let chat_id = ObjectId::new();
    let sender_id = ObjectId::new();

    // Create a message with an empty content string.
    let message = Message::new(chat_id, sender_id, "");

    // Check that the content is indeed empty.
    assert!(message.content.is_empty());
}
