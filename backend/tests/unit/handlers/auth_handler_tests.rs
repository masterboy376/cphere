use actix_web::{test, App};
use cphere_backend::handlers::auth::RegisterRequest;
use serde_json::json;

#[actix_web::test]
async fn test_register_handler() {
    // Build a test application with our auth routes.
    let app = test::init_service(App::new().configure(cphere_backend::handlers::init_routes)).await;
    
    // Create a JSON payload for registration.
    let payload = json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "secret"
    });
    
    // Create a POST request to /auth/register.
    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&payload)
        .to_request();
    
    // Send the request and receive the response.
    let resp = test::call_service(&app, req).await;
    
    // Check that the response has status 201 (Created).
    assert_eq!(resp.status(), 201);
    
    // Optionally, you can read the response body and check it.
    let resp_body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(resp_body, payload);
}
