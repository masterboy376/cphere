// handlers related unit tests
#[path = "unit/handlers/auth_handler_tests.rs"]
mod auth_handler_tests;
#[path = "unit/handlers/chat_handler_tests.rs"]
mod chat_handler_tests;
#[path = "unit/handlers/video_call_handler_tests.rs"]
mod video_call_handler_tests;


// middleware related unit tests
#[path = "unit/middlewares/rate_limiter_middleware_tests.rs"]
mod rate_limiter_middleware_tests;
#[path = "unit/middlewares/session_middleware_tests.rs"]
mod session_middleware_tests;


// models related unit tests
#[path = "unit/models/chat_model_tests.rs"]
mod chat_model_tests;
#[path = "unit/models/message_model_tests.rs"]
mod message_model_tests;
#[path = "unit/models/notification_model_tests.rs"]
mod notification_model_tests;
#[path = "unit/models/session_model_tests.rs"]
mod session_model_tests;
#[path = "unit/models/user_model_tests.rs"]
mod user_model_tests;


// service related unit tests
#[path = "unit/services/auth_service_tests.rs"]
mod auth_service_tests;
#[path = "unit/services/chat_service_tests.rs"]
mod chat_service_tests;
#[path = "unit/services/notification_service_tests.rs"]
mod notification_service_tests;
#[path = "unit/services/session_service_tests.rs"]
mod session_service_tests;


// utils related unit tests
#[path = "unit/utils/auth_util_tests.rs"]
mod auth_util_tests;
#[path = "unit/utils/validation_util_tests.rs"]
mod validation_util_tests;