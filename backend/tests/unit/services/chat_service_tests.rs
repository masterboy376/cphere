use cphere_backend::services::chat;
use mongodb::bson::oid::ObjectId;
use chrono::Utc;

#[tokio::test]
async fn test_create_chat_success() {
    // Create dummy participant IDs.
    let participant1 = ObjectId::new();
    let participant2 = ObjectId::new();
    let participants = vec![participant1.clone(), participant2.clone()];

    let result = chat::create_chat(participants.clone()).await;
    assert!(result.is_ok(), "Chat creation should succeed");

    let chat = result.unwrap();
    // The Chat model stores participants in a vector.
    assert_eq!(chat.participants, participants);
    // Ensure the created_at timestamp is valid.
    assert!(chat.created_at <= Utc::now());
}

#[actix_web::test]
async fn test_send_message_service() {
    let chat_id = ObjectId::new();
    let sender_id = ObjectId::new();
    let content = "Hello WebSocket!";
    
    let result = chat::send_message(chat_id, sender_id, content).await;
    assert!(result.is_ok());
    let message = result.unwrap();
    assert_eq!(message.content, content);
    assert_eq!(message.chat_id, chat_id);
    assert_eq!(message.sender_id, sender_id);
    assert!(message.created_at <= Utc::now());
}