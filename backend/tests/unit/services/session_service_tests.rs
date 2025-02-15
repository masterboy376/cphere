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
