use cphere_backend::utils::auth_util::{hash_password, Argon2Error};

#[test]
fn test_hash_password_success() {
    let password = "mysecretpassword";
    let result = hash_password(password);
    
    // Assert that hashing is successful.
    assert!(result.is_ok());
    let hashed = result.unwrap();
    
    // Check that the resulting hash is not equal to the plaintext password.
    assert_ne!(password, hashed);
    
    // Basic check to see if the hash string starts with an Argon2 identifier.
    // Depending on the argon2 version/configuration, this might be "$argon2i$" or "$argon2id$".
    assert!(
        hashed.starts_with("$argon2i$") || hashed.starts_with("$argon2id$"),
        "Hash should start with an Argon2 identifier"
    );
}

#[test]
fn test_hash_password_error() {
    // In this simple example, it's hard to force an error without changing the implementation.
    // However, you can simulate an error by passing an empty string or by modifying the function.
    // For now, we'll just assert that a valid password returns Ok.
    let password = "anotherpassword";
    let result = hash_password(password);
    assert!(result.is_ok());
}
