use cphere_backend::middleware::session::{SessionValidation, RequestContext, Middleware, MiddlewareError};
use mongodb::bson::oid::ObjectId;

#[tokio::test]
async fn test_session_validation_success() {
    let mut context = RequestContext {
        session_id: Some(ObjectId::new()),
    };
    let middleware = SessionValidation;
    let result = middleware.handle(&mut context).await;
    assert!(result.is_ok(), "Session should be valid when session_id exists");
}

#[tokio::test]
async fn test_session_validation_failure() {
    let mut context = RequestContext {
        session_id: None,
    };
    let middleware = SessionValidation;
    let result = middleware.handle(&mut context).await;
    // We expect a SessionInvalid error when no session_id is provided.
    assert!(matches!(result, Err(MiddlewareError::SessionInvalid)));
}
