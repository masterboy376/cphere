use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_session::config::PersistentSession;
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    web::{self}, App, HttpServer,
};
use actix_cors::Cors;
use cphere_backend::{
    config::database::init_db,
    handlers::{
        auth_handler::{
            change_password_handler, login_handler, logout_handler, auth_status_handler, register_handler,
            reset_password_handler,
        },
        chat_handler::{
            create_new_chat_handler, get_chat_messages_handler, send_message_handler, delete_chat_handler
        },
        ws_handler::ws_session_start_handler,
        user_handler::{
            check_batch_online_handler, check_online_handler, get_chats_handler, get_my_data_handler, get_notifications_handler, search_users_handler
        },
        video_call_handler::{initiate_video_call, respond_video_call},
    },
    middleware::auth_middleware::AuthMiddlewareFactory,
    states::app_state::AppState,
};
use env_logger;
use mongodb::{Client, Database};
use time::Duration;
use tokio::signal;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();

    // Initialize the database
    let (client, db): (Client, Database) = match init_db().await {
        Ok((client, db)) => (client, db),
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database initialization failed",
            ));
        }
    };

    // Initialize AppState with the database
    let app_state = AppState::new(client, db);

    // Wrap AppState in web::Data to make it shareable
    let app_state_data = web::Data::new(app_state);

    // Start the Actix server
    println!("Server running on http://127.0.0.1:8080");

    // Create the server instance without immediately `.await`ing it
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state_data.clone()) // Add AppState to the app's data
            .wrap(Logger::default()) // Enable logging middleware
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                        actix_web::http::header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(&[0; 64]), // Use a 64-byte key
                )
                .cookie_secure(false) // Set to true in production with HTTPS
                .cookie_name("session_id".to_owned()) // Optionally set a custom cookie name
                .cookie_same_site(SameSite::Lax) // Set the SameSite policy
                .cookie_http_only(true) // Prevent JavaScript access (security best practice)
                .session_lifecycle(
                    PersistentSession::default().session_ttl(Duration::days(7)), // Keep session for 7 days
                )
                .build(),
            )
            .configure(|cfg| {
                cfg.service(
                    web::scope("/auth")
                        .service(register_handler)
                        .service(login_handler)
                        .service(logout_handler)
                        .service(auth_status_handler)
                        .service(reset_password_handler)
                        .service(change_password_handler),
                )
                .service(search_users_handler)
                .service(
                    web::scope("/socket")
                        .wrap(AuthMiddlewareFactory {}) // Instantiate the middleware
                        .service(ws_session_start_handler),
                )
                .service(
                    web::scope("/users")
                        .wrap(AuthMiddlewareFactory {}) // Instantiate the middleware
                        .service(get_chats_handler)
                        .service(check_online_handler)
                        .service(check_batch_online_handler)
                        .service(get_notifications_handler)
                        .service(get_my_data_handler)
                )
                .service(
                    web::scope("/chats")
                        .wrap(AuthMiddlewareFactory {}) // Instantiate the middleware
                        .service(create_new_chat_handler)
                        .service(delete_chat_handler)
                        .service(send_message_handler)
                        .service(get_chat_messages_handler),
                )
                .service(
                    web::scope("/video_call")
                        .wrap(AuthMiddlewareFactory {}) // Instantiate the middleware
                        .service(initiate_video_call)
                        .service(respond_video_call),
                );
            })
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    // Clone the server handle to use in the signal handler
    let server_handle = server.handle();

    // Spawn a task to handle shutdown signals
    let graceful = tokio::spawn(async move {
        // Wait for Ctrl+C signal
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        println!("Received Ctrl+C, shutting down server gracefully");

        // Signal the server to stop accepting new connections and finish existing ones
        server_handle.stop(true).await;
    });

    // Run the server
    let result = server.await;

    // Wait for the graceful shutdown task to complete
    let _ = graceful.await;

    result
}
