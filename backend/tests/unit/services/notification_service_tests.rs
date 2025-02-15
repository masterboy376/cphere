use cphere_backend::services::notification;
use mongodb::bson::oid::ObjectId;
use chrono::Utc;

#[tokio::test]
async fn test_send_notification_success() {
    let user_id = ObjectId::new();
    let message = "Test notification";
    let result = notification::send_notification(user_id, message).await;
    assert!(result.is_ok(), "Notification should be sent successfully");

    let notif = result.unwrap();
    assert_eq!(notif.user_id, user_id);
    assert_eq!(notif.message, message);
    // By default, the notification should be unread.
    assert_eq!(notif.read, false);
    assert!(notif.created_at <= Utc::now());
}
