use cphere_backend::models::session::Session;
use chrono::{Utc, Duration};
use mongodb::bson::oid::ObjectId;

#[test]
fn test_create_session() {
    let user_id = ObjectId::new();
    // Set the expiration time to 7 days from now.
    let expires_at = Utc::now() + Duration::days(7);

    let session = Session::new(user_id, expires_at);

    // Verify that the session's user_id and expires_at fields match.
    assert_eq!(session.user_id, user_id);
    assert_eq!(session.expires_at, expires_at);
}

#[test]
fn test_session_expires_in_future() {
    let user_id = ObjectId::new();
    let expires_at = Utc::now() + Duration::days(7);

    let session = Session::new(user_id, expires_at);

    // Confirm that the expiration time is indeed later than the current time.
    assert!(session.expires_at > Utc::now());
}
