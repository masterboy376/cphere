use actix_session::Session;
use actix_web::{post, web, HttpResponse};
use futures::stream::StreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::{
    models::user::User,
    states::app_state::AppState,
    utils::auth_util::{hash_password, verify_password},
};

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

#[post("/register")]
pub async fn register_handler(
    state: web::Data<AppState>,
    req: web::Json<RegisterRequest>
) -> Result<HttpResponse, actix_web::Error> {
    let hashed_password = match hash_password(&req.password) {
        Ok(hashed_password) => hashed_password,
        Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to hash password")),
    };
    let user = User::new(&req.username, &req.email, &hashed_password.to_string());

    let collection = state.db.collection::<User>(User::collection_name());
    collection.insert_one(user, None).await.map_err(|e| {
        eprintln!("Mongo error: {0}", e);
        actix_web::error::ErrorInternalServerError("Failed to register user")
    })?;
    
    Ok(HttpResponse::Ok().json("User registered successfully"))
}

#[post("/login")]
pub async fn login_handler(
    credentials: web::Json<LoginRequest>,
    session: Session,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let collection = state.db.collection::<User>(User::collection_name());
    let user = collection
        .find_one(doc! { "username": &credentials.username }, None)
        .await
        .map_err(|e| {
            eprintln!("MongoDB error: {}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?
        .ok_or_else(|| {
            actix_web::error::ErrorUnauthorized("Invalid credentials")
        })?;

    let is_password_valid = verify_password(&credentials.password, &user.password_hash)
        .unwrap_or(false);

    if is_password_valid {
        // Store user ID in the session
        session.insert("user_id", &user.id.expect("User has no ID").to_hex())?;
        Ok(HttpResponse::Ok().json("Login successful"))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Invalid credentials"))
    }
}

#[post("/logout")]
pub async fn logout_handler(
    session: Session
) -> Result<HttpResponse, actix_web::Error> {
    if let Some(user_id) = session.get::<String>("user_id")? {
        session.remove("user_id");
        Ok(HttpResponse::Ok().json("Logged out successfully"))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    }
}

// Function to register routes in this module.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register_handler);
    cfg.service(login_handler);
    cfg.service(logout_handler);
}
