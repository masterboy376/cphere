use cphere_backend::models::chat_model::Chat;
use mongodb::bson::oid::ObjectId;
use chrono::Utc;

#[test]
fn test_create_chat() {
    // Create two ObjectIds for participants.
    let participant1 = ObjectId::new();
    let participant2 = ObjectId::new();
    let participant_ids = vec![participant1.clone(), participant2.clone()];

    // Create a chat using the participants vector.
    let chat = Chat::new(None, participant_ids.clone(), None);

    // Check that the participants vector matches.
    assert_eq!(chat.participant_ids, participant_ids);

    // Verify that the created_at timestamp is not in the future.
    assert!(chat.created_at <= Utc::now());
}

