use crate::{
    services::auth_service::{
        authenticate_user, change_password, logout_user, register_user, send_reset_password_email, auth_status,
        ChangePasswordRequest, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, ResetPasswordRequest,
    },
    states::app_state::AppState,
};
use actix_session::Session;
use actix_web::{get, post, web, Error, HttpResponse};

#[post("/register")]
pub async fn register_handler(
    state: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let user = register_user(&state, &req).await?;

    if let Some(user_id) = &user.id {
        session.insert("user_id", &user_id.to_hex())?;
        session.renew();
        let result = RegisterResponse {
            user_id: user_id.to_string(),
            username: user.username
        };
        let response = serde_json::json!(result);
        
        Ok(HttpResponse::Ok().json(response))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Failed to resister user"))
    }
}

#[post("/login")]
pub async fn login_handler(
    state: web::Data<AppState>,
    req: web::Json<LoginRequest>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let user = authenticate_user(&req, &state).await?;

    // Store user ID in the session
    if let Some(user_id) = &user.id {
        session.insert("user_id", &user_id.to_hex())?;
        session.renew();
        let result = LoginResponse{
            user_id: user_id.to_string(),
            username: user.username
        };
        let response = serde_json::json!(result);
        Ok(HttpResponse::Ok().json(response))
    } else {
        Err(actix_web::error::ErrorUnauthorized("Invalid user ID"))
    }
}

#[post("/logout")]
pub async fn logout_handler(session: Session) -> Result<HttpResponse, Error> {
    let result = logout_user(&session).await?;
    Ok(HttpResponse::Ok().json(result))
}

#[get("/auth_status")]
pub async fn auth_status_handler(
    state: web::Data<AppState>,
    session: Session
) -> Result<HttpResponse, Error> {
    let result = auth_status(&state, &session).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!(result)))
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
