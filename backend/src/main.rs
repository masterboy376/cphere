use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{App, HttpServer, web, cookie::{Key, SameSite}};
use cphere_backend::{
    config::database::init_db,
    handlers::{
        auth::{login_handler, logout_handler, register_handler},
        chat::ws_session_start_handler,
    },
    middleware::auth_middleware::AuthMiddlewareFactory,
    states::app_state::AppState,
};
use mongodb::{Client, Database};
use std::sync::Arc;
use tokio::signal;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(&[0; 64]), // Use a 64-byte key
                )
                .cookie_secure(false) // Set to true in production with HTTPS
                .cookie_name("session_id".to_owned()) // Optionally set a custom cookie name
                .cookie_same_site(SameSite::Lax) // Set the SameSite policy
                .build(),
            )
            .configure(|cfg| {
                cfg.service(
                    web::scope("/auth")
                        .service(register_handler)
                        .service(login_handler)
                        .service(logout_handler),
                )
                .service(
                    web::scope("/ws")
                        .wrap(AuthMiddlewareFactory {}) // Instantiate the middleware
                        .service(ws_session_start_handler),
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
