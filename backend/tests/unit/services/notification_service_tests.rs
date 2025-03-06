use cphere_backend::services::notification_service;
use mongodb::bson::oid::ObjectId;
use chrono::Utc;

#[tokio::test]
async fn test_send_notification_success() {
    let recipient_id = ObjectId::new();
    let sender_id = ObjectId::new();
    let message = "Test notification";
    let result = notification_service::send_notification(
        recipient_id,
        sender_id,
        message).await;
    assert!(result.is_ok(), "Notification should be sent successfully");

    let notif = result.unwrap();
    assert_eq!(notif.recipient_id, recipient_id);
    assert_eq!(notif.message, message);
    // By default, the notification should be unread.
    assert_eq!(notif.is_handled, false);
    assert!(notif.created_at <= Utc::now());
}
