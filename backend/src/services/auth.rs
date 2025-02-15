// src/services/auth.rs
use crate::models::user::User;
use crate::utils::auth_util::{hash_password, Argon2Error}; // Assume we have this utility
use crate::config::database::{init_db, DbError};
use mongodb::error::Error as MongoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Database error: {0}")]
    Db(#[from] DbError),
    #[error("Password hashing error: {0}")]
    Hash(#[from] Argon2Error),
    #[error("MongoDB error: {0}")]
    Mongo(#[from] MongoError),
    #[error("User not found")]
    UserNotFound,
}

pub async fn register_user(username: &str, email: &str, password: &str) -> Result<User, AuthError> {
    // Hash the password
    let hashed_password = hash_password(password).map_err(|e| AuthError::Hash(e))?;
    let new_user = User::new(username, email, &hashed_password);

    let (client, db) = init_db().await?;
    let collection = db.collection::<User>("users");
    collection.insert_one(new_user.clone(), None).await?;

    Ok(new_user)
}
