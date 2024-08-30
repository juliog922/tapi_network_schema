use actix_web::{get, Error, HttpResponse, error};
use reqwest::Client;
use serde_json::Value;
use std::fs::File;
use std::io::Write;

use crate::utils::{
    matching, 
    to_list
};

/// Asynchronously fetches data from a remote REST API and saves different parts of the JSON response to separate files.
///
/// This handler function:
/// 1. Makes an HTTP GET request to the specified URL with the given headers and authentication.
/// 2. Extracts specific parts of the JSON response using utility functions.
/// 3. Serializes these parts into JSON strings.
/// 4. Writes each JSON string to a separate text file on the server.
/// 5. Returns an HTTP response indicating success or failure.
///
/// # Returns
///
/// * `Result<HttpResponse, Error>` - Returns an `HttpResponse` with a success message if the operation is successful,
///   or an `Error` if something goes wrong.
#[get("/fetch")]
async fn save_schema() -> Result<HttpResponse, Error> {
    // Create an HTTP client with customized settings
    let client = Client::builder()
        .danger_accept_invalid_certs(true) // Accept invalid SSL certificates (for testing)
        .gzip(true) // Enable gzip compression
        .brotli(true) // Enable Brotli compression
        .deflate(true) // Enable deflate compression
        .build()
        .unwrap();

    // Send an HTTP GET request to the specified URL
    let response = client
        .get("https://10.95.87.21:18010/restconf/data/tapi-common:context")
        .header("Accept", "application/yang-data+json")
        .header("Accept-Encoding", "gzip, deflate, br")
        .basic_auth("tapi", Some("Zenap_1235!!!")) // Basic authentication
        .send()
        .await
        .map_err(|_| error::ErrorNotFound("Request Error"))?;

    // Parse the response JSON
    let json: Value = response.json().await.map_err(|_| error::ErrorNotFound("Empty Response."))?;

    // Extract specific parts of the JSON response
    let connectivity_services = to_list(matching(true, &json, "/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")?)?;
    let connections = matching(true, &json, "/tapi-common:context/tapi-connectivity:connectivity-context/connection")?;
    let services_interface_point = to_list(matching(true, &json, "/tapi-common:context/service-interface-point")?)?;
    let topology = matching(true, &json, "/tapi-common:context/tapi-topology:topology-context/topology")?;

    // Serialize each part into a JSON string
    let connectivity_services_str = serde_json::to_string(&connectivity_services).map_err(|_| error::ErrorInternalServerError("Serialization Error"))?;
    let connections_str = serde_json::to_string(&connections).map_err(|_| error::ErrorInternalServerError("Serialization Error"))?;
    let services_interface_point_str = serde_json::to_string(&services_interface_point).map_err(|_| error::ErrorInternalServerError("Serialization Error"))?;
    let topology_str = serde_json::to_string(&topology).map_err(|_| error::ErrorInternalServerError("Serialization Error"))?;

    // Write each JSON string to a separate file
    let mut file = File::create("connectivity_services.txt").map_err(|_| error::ErrorInternalServerError("File Creation Error"))?;
    file.write_all(connectivity_services_str.as_bytes()).map_err(|_| error::ErrorInternalServerError("File Write Error"))?;

    let mut file = File::create("connections.txt").map_err(|_| error::ErrorInternalServerError("File Creation Error"))?;
    file.write_all(connections_str.as_bytes()).map_err(|_| error::ErrorInternalServerError("File Write Error"))?;

    let mut file = File::create("services_interface_point.txt").map_err(|_| error::ErrorInternalServerError("File Creation Error"))?;
    file.write_all(services_interface_point_str.as_bytes()).map_err(|_| error::ErrorInternalServerError("File Write Error"))?;

    let mut file = File::create("topology.txt").map_err(|_| error::ErrorInternalServerError("File Creation Error"))?;
    file.write_all(topology_str.as_bytes()).map_err(|_| error::ErrorInternalServerError("File Write Error"))?;

    // Return a success message
    Ok(HttpResponse::Ok().body("Data saved successfully"))
}
