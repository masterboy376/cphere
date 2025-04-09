use argon2::{
    password_hash::{Error as PHError, PasswordHash, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use rand::{rngs::OsRng, Rng};
use std::error::Error;
use crate::config::app_config::AppConfig;

pub fn hash_password(password: &str) -> Result<String, PHError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    // This will return Result<PasswordHash, PHError>
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, PHError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?; // Convert hash string to PasswordHash
    let is_valid = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    Ok(is_valid)
}

pub fn generate_reset_token() -> String {
    let config = AppConfig::new().unwrap();
    let mut rng = rand::thread_rng();
    (0..config.reset_token_length)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect()
}

pub fn send_reset_email(email: &str, token: &str) -> Result<(), Box<dyn Error>> {
    let config = AppConfig::new().unwrap();
    
    let reset_link = format!("http://localhost:5173/reset-password/{}", token);
    // reset_link = format!("{}/reset_password?email={}&token={}", config.frontend_url, email, token);
    // Build the email message
    let email_message = Message::builder()
        .from(config.email_address.parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Password Reset")
        .body(format!("Visit this link to reset your password: {}", reset_link))?;
    
    // Create credentials using your Gmail address and app password.
    let creds = Credentials::new(
        config.email_address.clone(),
        config.email_password.clone(),  // Replace with your actual app password
    );
    
    // Build an SMTP transport using Gmail's SMTP server.
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();
    
    // Send the email.
    mailer.send(&email_message)?;
    Ok(())
}
