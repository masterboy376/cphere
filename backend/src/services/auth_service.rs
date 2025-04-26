use crate::{
    models::user_model::User,
    states::app_state::AppState,
    utils::auth_util::{generate_reset_token, hash_password, send_reset_email, verify_password},
    config::app_config::AppConfig,
    services::user_service::{extract_user_id_from_session, get_user_by_id},
};
use actix_session::Session;
use actix_web::{web, Error};
use chrono::{Duration, Utc};
use mongodb::bson::{doc, Bson};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub reset_token: String,
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthStatusResponse {
    pub user_id: Option<String>,
    pub username: Option<String>,
}

pub async fn register_user(
    state: &web::Data<AppState>,
    req: &RegisterRequest,
) -> Result<User, Error> {
    let user_collection = state.db.collection::<User>(User::collection_name());
    
    // Check if username or email already exists
    let existing_user = user_collection
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

    // Create the user
    let hashed_password = hash_password(&req.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to hash password"))?;
    let mut user = User::new(&req.username, &req.email, &hashed_password);
    let insert_result = user_collection.insert_one(&user, None).await.map_err(|e| {
        log::error!("MongoDB error: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to register user")
    })?;
    user.id = insert_result.inserted_id.as_object_id();

    Ok(user)
}

pub async fn authenticate_user(
    credentials: &LoginRequest,
    state: &web::Data<AppState>,
) -> Result<User, Error> {
    let user_collection = state.db.collection::<User>(User::collection_name());
    let user = user_collection
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

pub async fn auth_status(
    state: &web::Data<AppState>,
    session: &Session,
) -> Result<AuthStatusResponse, Error> {
    let user_id_result = extract_user_id_from_session(session);
    
    match user_id_result {
        Ok(user_id) => {
            let user_option: Option<User> = get_user_by_id(state, user_id).await.ok();
                
            if let Some(user) = user_option {
                if user.id.is_none() {
                    return Err(actix_web::error::ErrorInternalServerError("User ID is None"));
                }
                Ok(AuthStatusResponse {
                    user_id: Some(user.id.unwrap().to_hex()),
                    username: Some(user.username),
                })
            } else {
                session.purge();
                Ok(AuthStatusResponse {
                    user_id: None,
                    username: None,
                })
            }
        },
        Err(_) => {
            Ok(AuthStatusResponse {
                user_id: None,
                username: None,
            })
        }
    }
}

pub async fn send_reset_password_email(
    state: &web::Data<AppState>,
    req: &ResetPasswordRequest,
) -> Result<String, Error> {
    let config = AppConfig::new().unwrap();
    let user_collection = state.db.collection::<User>(User::collection_name());
    let user = user_collection
        .find_one(doc! { "email": &req.email }, None)
        .await
        .map_err(|e| {
            log::error!("MongoDB error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Email not found"))?;
    let reset_token = generate_reset_token();
    let expires_at = Utc::now() + Duration::minutes(config.reset_token_expiration_minutes);
    user_collection
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

    Ok("Reset password email sent".to_string())
}

pub async fn change_password(
    state: &web::Data<AppState>,
    req: &ChangePasswordRequest,
) -> Result<String, Error> {
    let user_collection = state.db.collection::<User>(User::collection_name());

    let user = user_collection
        .find_one(doc! { "reset_token": &req.reset_token }, None)
        .await
        .map_err(|e| {
            log::error!("MongoDB error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid reset token"))?;

    if user.reset_token_expiry_at.unwrap_or_else(|| 0) < Utc::now().timestamp_millis() {
        return Err(actix_web::error::ErrorBadRequest("Reset token expired"));
    }

    let hashed_password = hash_password(&req.new_password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to hash password"))?;

    user_collection
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
