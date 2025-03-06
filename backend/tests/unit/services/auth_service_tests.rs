use cphere_backend::services::auth_service;
use cphere_backend::models::user_model::User;
use chrono::Utc;

// #[tokio::test]
// async fn test_register_user_success() {
//     // This test assumes a test-friendly DB configuration.
//     let result = auth_services::register_user("testuser", "test@example.com", "password123").await;
//     assert!(result.is_ok(), "Expected successful user registration");
    
//     let user = result.unwrap();
//     assert_eq!(user.username, "testuser");
//     assert_eq!(user.email, "test@example.com");
//     // Check that the creation time is not in the future.
//     assert!(user.created_at <= Utc::now());
// }
