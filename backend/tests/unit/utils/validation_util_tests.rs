// tests/unit/validation_tests.rs
use cphere_backend::utils::validation_util::{validate_email, validate_username};

#[test]
fn test_validate_email() {
    // Valid email should return true.
    assert!(validate_email("test@example.com"));
    // Invalid emails should return false.
    assert!(!validate_email("invalid-email"));
    assert!(!validate_email("test@.com"));
    assert!(!validate_email("test@example"));
}

#[test]
fn test_validate_username() {
    // Valid usernames.
    assert!(validate_username("user123"));
    assert!(validate_username("user_name"));
    // Too short.
    assert!(!validate_username("ab"));
    // Contains invalid characters.
    assert!(!validate_username("user!name"));
}
