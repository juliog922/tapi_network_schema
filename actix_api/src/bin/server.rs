use actix_cors::Cors;
use actix_web::{main, web, App, HttpServer};
use actix_api::logic::requester::DataSource;
use std::collections::HashMap;
use std::io::Result;
use std::sync::Arc;
use std::env;
use dotenv::dotenv;
use tokio::sync::Mutex;



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
    dotenv().ok();

    let port: u16 = env::var("API_PORT").ok().and_then(|val| {
        val.parse().ok()
    }).unwrap_or(8080);
    let host: String = env::var("API_HOST").unwrap_or("0.0.0.0".to_string());

    let host_dictionary: Arc<Mutex<HashMap<String, DataSource>>> =
        Arc::new(Mutex::new(HashMap::new()));
    // Create a shared, thread-safe dictionary to hold host parameters
    //let host_dictionary: Arc<Mutex<HashMap<String, HostParameters>>> = Arc::new(Mutex::new(HashMap::new()));

    // Start the HTTP server
    HttpServer::new(move || {
        // Configure CORS to be permissive
        let cors = Cors::permissive();

        App::new()
            .wrap(cors) // Apply CORS configuration
            .app_data(web::Data::new(host_dictionary.clone())) // Share `host_dictionary` with application
            .service(actix_api::routes::fetch::save_schema) // Register route for fetching and saving schema
            .service(actix_api::routes::get_services::connectivity_services) // Register route for connectivity services
            .service(actix_api::routes::get_schema::schema_by_service)
            .service(actix_api::routes::get_hosts::get_hosts) // Register route for getting hosts
            .service(actix_api::routes::add_host::add_host) // Register route for adding a new host
            .service(actix_api::routes::delete_host::delete_host) // Register route for deleting a host
            .service(actix_api::routes::by_files::upload_services)
    })
    .bind(
        (host.as_str(), port)
    )? // Bind the server to 0.0.0.0:8080
    .run() // Start the server
    .await // Await the server's completion
}

#[cfg(test)]
#[actix_web::test]
async fn test_server() {
    use actix_web::test;

    let host_dictionary: Arc<Mutex<HashMap<String, DataSource>>> =
    Arc::new(Mutex::new(HashMap::new()));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(host_dictionary.clone()))
            .service(actix_api::routes::get_hosts::get_hosts)
    )
    .await;

    let req = test::TestRequest::get().uri("/get_hosts").to_request();

    let resp = test::call_service(&app, req).await;
    println!("{}", resp.status());
    assert!(resp.status().is_success());

}
