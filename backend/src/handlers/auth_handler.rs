use crate::{
    services::auth_service::{
        authenticate_user, change_password, logout_user, register_user, send_reset_password_email,
        ChangePasswordRequest, LoginRequest, RegisterRequest, ResetPasswordRequest,
    },
    states::app_state::AppState,
};
use actix_session::Session;
use actix_web::{post, web, Error, HttpResponse};

#[post("/register")]
pub async fn register_handler(
    state: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
) -> Result<HttpResponse, Error> {
    let result = register_user(&state, &req).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/login")]
pub async fn login_handler(
    state: web::Data<AppState>,
    credentials: web::Json<LoginRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let user = authenticate_user(&credentials, &state).await?;

    // Store user ID in the session
    if let Some(user_id) = &user.id {
        session.insert("user_id", &user_id.to_hex())?;
        Ok(HttpResponse::Ok().json("Login successful"))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Invalid user ID"))
    }
}

#[post("/logout")]
pub async fn logout_handler(session: Session) -> Result<HttpResponse, Error> {
    logout_user(&session).await?;
    Ok(HttpResponse::Ok().json("Logged out successfully"))
}

#[post("/reset_password")]
pub async fn reset_password_handler(
    state: web::Data<AppState>,
    req: web::Json<ResetPasswordRequest>,
) -> Result<HttpResponse, Error> {
    let result = send_reset_password_email(&state, &req).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/change_password")]
pub async fn change_password_handler(
    state: web::Data<AppState>,
    req: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse, Error> {
    let result = change_password(&state, &req).await?;
    Ok(HttpResponse::Ok().json(result))
}
