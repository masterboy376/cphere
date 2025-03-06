use cphere_backend::models::user_model::User;
use chrono::Utc;
use validator::Validate;

#[test]
fn test_create_user() {
    let user = User::new("testuser", "test@example.com", "hashed_password");
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
    assert!(user.created_at <= Utc::now());
}

#[test]
fn test_user_validation() {
    let user = User {
        id: None,
        username: "te".to_string(), // Too short
        email: "invalid-email".to_string(), // Invalid email
        password_hash: "short".to_string(),
        created_at: Utc::now(),
    };

    let validation = user.validate();
    assert!(validation.is_err());
}
