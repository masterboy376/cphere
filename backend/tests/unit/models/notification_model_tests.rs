use cphere_backend::{
    models::notification_model::Notification,
    config::app_config::AppConfig
};
use mongodb::bson::oid::ObjectId;
use chrono::Utc;

use crate::session_middleware_tests;

#[test]
fn test_create_notification() {
    let config = AppConfig::new();

    // Create an ObjectId for the user.
    let recipient_id = ObjectId::new();
    let sender_id = ObjectId::new();

    // Create a notification using the expected parameters.
    let notification = Notification::new(
        config.unwrap().video_call_notification,
        recipient_id,
        sender_id,
        "New message");

    // Check that the recipient_id and message are set as expected.
    assert_eq!(notification.recipient_id, recipient_id);
    assert_eq!(notification.message, "New message");

    // The read flag should be false by default.
    assert_eq!(notification.is_handled, false);

    // Verify that the created_at timestamp is not in the future.
    assert!(notification.created_at <= Utc::now());
}
