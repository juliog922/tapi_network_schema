use actix_web::{get, Error, HttpResponse, error};
use reqwest::Client;
use serde_json::Value;
use std::fs::File;
use std::io::Write;

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
        .basic_auth("123", Some("Zte2024!")) // Basic authentication
        .send()
        .await
        .map_err(|_| error::ErrorNotFound("Request Error"))?;

    // Parse the response JSON
    let json: Value = response.json().await.map_err(|_| error::ErrorNotFound("Empty Response."))?;

    // Extract specific parts of the JSON response
    let connectivity_services = json.pointer("/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")
                                                        .ok_or_else(|| error::ErrorNotFound("Cannot find connectivity-service"))?
                                                        .as_array().ok_or_else(|| error::ErrorNotAcceptable("connectivity-service is not and array"))?.clone();
    let connections = json.pointer("/tapi-common:context/tapi-connectivity:connectivity-context/connection")
                                                        .ok_or_else(|| error::ErrorNotFound("Cannot find connection"))?
                                                        .as_array().ok_or_else(|| error::ErrorNotAcceptable("connection is not and array"))?.clone();
    let services_interface_point = json.pointer("/tapi-common:context/service-interface-point")
                                                        .ok_or_else(|| error::ErrorNotFound("Cannot find service-interface-point"))?
                                                        .as_array().ok_or_else(|| error::ErrorNotAcceptable("service-interface-point is not and array"))?.clone();
    let topology = json.pointer("/tapi-common:context/tapi-topology:topology-context/topology")
                                                    .ok_or_else(|| error::ErrorNotFound("Cannot find topology"))?.clone();

    // Serialize each part into a JSON string
    let connectivity_services_str = serde_json::to_string(&connectivity_services).map_err(|_| error::ErrorInternalServerError("Serialization Error"))?;
    let connections_str = serde_json::to_string(&connections).map_err(|_| error::ErrorInternalServerError("Serialization Error"))?;
    let services_interface_point_str = serde_json::to_string(&services_interface_point).map_err(|_| error::ErrorInternalServerError("Serialization Error"))?;
    let topology_str = serde_json::to_string(&topology).map_err(|_| error::ErrorInternalServerError("Serialization Error"))?;

    // Write each JSON string to a separate file
    let mut file = File::create("connectivity_services.json").map_err(|_| error::ErrorInternalServerError("File Creation Error"))?;
    file.write_all(connectivity_services_str.as_bytes()).map_err(|_| error::ErrorInternalServerError("File Write Error"))?;

    let mut file = File::create("connections.json").map_err(|_| error::ErrorInternalServerError("File Creation Error"))?;
    file.write_all(connections_str.as_bytes()).map_err(|_| error::ErrorInternalServerError("File Write Error"))?;

    let mut file = File::create("services_interface_point.json").map_err(|_| error::ErrorInternalServerError("File Creation Error"))?;
    file.write_all(services_interface_point_str.as_bytes()).map_err(|_| error::ErrorInternalServerError("File Write Error"))?;

    let mut file = File::create("topology.json").map_err(|_| error::ErrorInternalServerError("File Creation Error"))?;
    file.write_all(topology_str.as_bytes()).map_err(|_| error::ErrorInternalServerError("File Write Error"))?;

    // Return a success message
    Ok(HttpResponse::Ok().body("Data saved successfully"))
}
