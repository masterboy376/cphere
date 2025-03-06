use crate::{
    services::{
        chat_service::get_user_chats,
        notification_service::get_user_notifications,
        user_service::{
            extract_user_id_from_session, get_user_data, is_user_online, search_users,
            BatchCheckOnlineRequest, BatchCheckOnlineResponse,
        },
    },
    states::app_state::AppState,
};
use actix_session::SessionExt;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use futures;
use mongodb::bson::oid::ObjectId;

#[get("/{user_id}/is_online")]
pub async fn check_online_handler(
    _req: HttpRequest, // Authentication handled by middleware
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_id_str = path.into_inner();
    let user_id = ObjectId::parse_str(&user_id_str)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;

    let is_online = is_user_online(&state, user_id).await;

    Ok(HttpResponse::Ok().json(is_online))
}

#[post("/is_batch_online")]
pub async fn check_batch_online_handler(
    _req: HttpRequest, // Authentication handled by middleware
    state: web::Data<AppState>,
    body: web::Json<BatchCheckOnlineRequest>,
) -> Result<HttpResponse, Error> {
    let user_ids = body
        .user_ids
        .iter()
        .filter_map(|id| ObjectId::parse_str(id).ok())
        .collect::<Vec<ObjectId>>();

    let online_status = futures::future::join_all(user_ids.into_iter().map(|id| {
        let state = state.clone();
        async move {
            let is_online = is_user_online(&state, id).await;
            (id.to_hex(), is_online)
        }
    }))
    .await;

    Ok(HttpResponse::Ok().json(BatchCheckOnlineResponse { online_status }))
}

#[get("/chats")]
pub async fn get_chats_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Get user ID from session
    let session = req.get_session();
    let user_id = extract_user_id_from_session(&session)?;
    // Get the user's chats
    let results = get_user_chats(&state, user_id).await?;

    Ok(HttpResponse::Ok().json(results))
}

#[get("/get_notifications")]
pub async fn get_notifications_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Get user ID from session
    let session = req.get_session();
    let user_id = extract_user_id_from_session(&session)?;
    // Get the user's notifications
    let results = get_user_notifications(&state, user_id).await?;

    Ok(HttpResponse::Ok().json(results))
}

#[get("/search_users")]
pub async fn search_users_handler(
    state: web::Data<AppState>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    // Expect a query parameter named "q"
    let q = query
        .get("q")
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing search query parameter 'q'"))?;

    // Call the user service to perform the search.
    let results = search_users(&state, q).await?;
    Ok(HttpResponse::Ok().json(results))
}

// Fetch logged-in user’s data using session information
#[get("/me")]
pub async fn get_my_data_handler(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let session = req.get_session();

    // Call the user service to get the user's data.
    let user_json = get_user_data(&state, &session).await?;
    Ok(HttpResponse::Ok().json(user_json))
}
