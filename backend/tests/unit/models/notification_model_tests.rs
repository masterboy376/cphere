use cphere_backend::models::notification::Notification;
use mongodb::bson::oid::ObjectId;
use chrono::Utc;

#[test]
fn test_create_notification() {
    // Create an ObjectId for the user.
    let user_id = ObjectId::new();

    // Create a notification using the expected parameters.
    let notification = Notification::new(user_id, "New message");

    // Check that the user_id and message are set as expected.
    assert_eq!(notification.user_id, user_id);
    assert_eq!(notification.message, "New message");

    // The read flag should be false by default.
    assert_eq!(notification.read, false);

    // Verify that the created_at timestamp is not in the future.
    assert!(notification.created_at <= Utc::now());
}
