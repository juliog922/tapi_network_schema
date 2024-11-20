mod routes;
mod utils;
mod pross;

use actix_web::{web, main, App, HttpServer};
use tokio::sync::Mutex;
use std::io::Result;
use actix_cors::Cors;
use std::collections::HashMap;
use std::sync::Arc;

/// Structure to hold host parameters including port, user, and password.
#[derive(Clone, Debug)]
pub struct HostParameters {
    /// The port number for the host.
    port: Option<String>,
    /// The tenant for the host.
    tenant: Option<String>,
    /// The username for the host.
    user: String,
    /// The password for the host.
    password: String,
}

/// Main entry point for the Actix web server.
///
/// This function sets up and runs the Actix web server with the following configurations:
/// 1. **CORS Configuration:** Permissive CORS setup to allow requests from any origin.
/// 2. **Data Sharing:** Uses `web::Data` to share the `host_dictionary` across the application.
/// 3. **Routes:** Registers various routes for handling different types of HTTP requests.
///
/// # Returns
///
/// * `Result<()>` - Returns a `Result` indicating success or failure. 
///   - On success, the server starts and runs.
///   - On failure, an `io::Error` is returned.
#[main]
async fn main() -> Result<()> {
    // Create a shared, thread-safe dictionary to hold host parameters
    let host_dictionary: Arc<Mutex<HashMap<String, HostParameters>>> = Arc::new(Mutex::new(HashMap::new()));

    // Start the HTTP server
    HttpServer::new(move || {
        // Configure CORS to be permissive
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors) // Apply CORS configuration
            .app_data(web::Data::new(host_dictionary.clone())) // Share `host_dictionary` with application
            .service(crate::routes::fetch::save_schema) // Register route for fetching and saving schema
            .service(crate::routes::services::connectivity_services) // Register route for connectivity services
            .service(crate::routes::get_hosts::get_hosts) // Register route for getting hosts
            .service(crate::routes::add_host::add_host) // Register route for adding a new host
            .service(crate::routes::delete_host::delete_host) // Register route for deleting a host
    })
    .bind(("0.0.0.0", 8080))? // Bind the server to 0.0.0.0:8080
    .run() // Start the server
    .await // Await the server's completion
}
