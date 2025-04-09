use crate::{
    models::user_model::User,
    states::app_state::AppState,
    utils::auth_util::{generate_reset_token, hash_password, send_reset_email, verify_password},
    config::app_config::AppConfig,
};
use actix_session::Session;
use actix_web::{web, Error};
use chrono::{Duration, Utc};
use mongodb::bson::{doc, Bson};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangePasswordRequest {
    pub reset_token: String,
    pub new_password: String,
}

pub async fn register_user(
    state: &web::Data<AppState>,
    req: &RegisterRequest,
) -> Result<User, Error> {
    // Check if username or email already exists
    let collection = state.db.collection::<User>(User::collection_name());

    let existing_user = collection
        .find_one(
            doc! { "$or": [ { "username": &req.username }, { "email": &req.email } ] },
            None,
        )
        .await
        .map_err(|e| {
            log::error!("MongoDB error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if existing_user.is_some() {
        return Err(actix_web::error::ErrorBadRequest(
            "Username or email already exists",
        ));
    }

    // Hash the password
    let hashed_password = hash_password(&req.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to hash password"))?;

    // Create the user
    let mut user = User::new(&req.username, &req.email, &hashed_password);

    // Insert into the database
    let insert_result = collection.insert_one(&user, None).await.map_err(|e| {
        log::error!("MongoDB error: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to register user")
    })?;

    // Update the user with its newly generated id, if applicable.
    // Change this assignment as needed based on your User id type.
    user.id = insert_result.inserted_id.as_object_id();

    Ok(user)
}

pub async fn authenticate_user(
    credentials: &LoginRequest,
    state: &web::Data<AppState>,
) -> Result<User, Error> {
    let collection = state.db.collection::<User>(User::collection_name());

    let user = collection
        .find_one(doc! { "username": &credentials.username }, None)
        .await
        .map_err(|e| {
            log::error!("MongoDB error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid credentials"))?;

    let is_password_valid = verify_password(&credentials.password, &user.password_hash)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to verify password"))?;

    if is_password_valid {
        Ok(user)
    } else {
        Err(actix_web::error::ErrorUnauthorized("Invalid credentials"))
    }
}

pub async fn logout_user(
    session: &Session
) -> Result<String, Error> {
    session.purge();
    Ok("Logged out successfully".to_string())
}

pub async fn send_reset_password_email(
    state: &web::Data<AppState>,
    req: &ResetPasswordRequest,
) -> Result<String, Error> {
    let config = AppConfig::new().unwrap();
    let collection = state.db.collection::<User>(User::collection_name());

    let user = collection
        .find_one(doc! { "email": &req.email }, None)
        .await
        .map_err(|e| {
            log::error!("MongoDB error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Email not found"))?;

    let reset_token = generate_reset_token();
    let expires_at = Utc::now() + Duration::minutes(config.reset_token_expiration_minutes);

    collection
        .update_one(
            doc! { "_id": &user.id },
            doc! { "$set": { "reset_token": &reset_token, "reset_token_expiry_at": expires_at.timestamp_millis() } },
            None,
        )
        .await
        .map_err(|e| {
            log::error!("MongoDB error: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to set reset token")
        })?;

    send_reset_email(&req.email, &reset_token).map_err(|e| {
        log::error!("Email error: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to send reset email")
    })?;

    Ok("Reset password email sent successfully".to_string())
}

pub async fn change_password(
    state: &web::Data<AppState>,
    req: &ChangePasswordRequest,
) -> Result<String, Error> {
    let collection = state.db.collection::<User>(User::collection_name());

    let user = collection
        .find_one(doc! { "reset_token": &req.reset_token }, None)
        .await
        .map_err(|e| {
            log::error!("MongoDB error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid reset token"))?;

    // Check if token is expired
    if user.reset_token_expiry_at.unwrap_or_else(|| 0) < Utc::now().timestamp_millis() {
        return Err(actix_web::error::ErrorBadRequest("Reset token expired"));
    }

    // Hash the new password
    let hashed_password = hash_password(&req.new_password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to hash password"))?;

    collection
        .update_one(
            doc! { "_id": &user.id },
            doc! { "$set": { "password_hash": &hashed_password, "reset_token": "", "reset_token_expiry_at": Bson::Null } },
            None,
        )
        .await
        .map_err(|e| {
            log::error!("MongoDB error: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to update password")
        })?;

    Ok("Password changed successfully".to_string())
}
