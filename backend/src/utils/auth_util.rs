use argon2::{Argon2, PasswordHasher, password_hash::{Error as PHError, SaltString, PasswordHash}, PasswordVerifier};
use rand::rngs::OsRng;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Argon2Error {
    #[error("Password hashing error: {0}")]
    Hash(String),
}

impl From<PHError> for Argon2Error {
    fn from(err: PHError) -> Self {
        // Convert the PHError nto a string message.
        Argon2Error::Hash(err.to_string())
    }
}

pub fn hash_password(password: &str) -> Result<String, Argon2Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    // This will return Result<PasswordHash, PHError>
    let hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, PHError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?;  // Convert hash string to PasswordHash
    let is_valid = argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok();
    Ok(is_valid)
}
