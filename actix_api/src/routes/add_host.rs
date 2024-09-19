use actix_web::{error, post, web, Error, HttpResponse};
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::json;
use std::sync::{Arc, Mutex};
use ipaddress::IPAddress;

use crate::HostParameters;

/// Struct representing the request body for adding a new host.
#[derive(Debug, Clone, Deserialize)]
struct AddHostRequest {
    host: String,
    port: Option<String>,
    tenant: Option<String>,
    user: String,
    password: String,
}

/// HTTP POST endpoint to add a new host to the host dictionary.
/// 
/// # Arguments
/// 
/// * `host_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>` representing the host dictionary.
/// * `request` - A `web::Json<AddHostRequest>` representing the request body containing host details.
/// 
/// # Returns
/// 
/// An `impl Responder` containing an `HttpResponse` indicating the result of the operation.
#[post("/add_host")]
pub async fn add_host(
    host_dictionary: web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>, 
    request: web::Json<AddHostRequest>
) -> Result<HttpResponse, Error> {
    // Validate the IP address or hostname
    if let Err(_) = validate_host(&request.host) {
        return Err(error::ErrorBadRequest("Host cannot be added"));
    }

    // Lock the host dictionary for writing.
    let mut host_dictionary = host_dictionary.lock().unwrap();
    // Create a new HostParameters instance from the request data.
    let host_parameters = HostParameters {
        port: request.port.clone(),
        tenant: request.tenant.clone(),
        user: request.user.clone(),
        password: request.password.clone(),
    };
    // Insert the new host into the dictionary.
    host_dictionary.insert(request.host.clone(), host_parameters);
    // Return an HTTP response indicating successful addition.
    Ok(HttpResponse::Ok().json(json!({"message": &format!("{} added successfully", request.host)})))
}

/// Validate if the provided host is a valid IP address or hostname.
fn validate_host(host: &str) -> Result<(), &'static str> {
    // Check if the host is a valid IP address.
    if IPAddress::parse(host).is_ok() {
        return Ok(());
    }

    Err("Invalid IP address or hostname.")
}
