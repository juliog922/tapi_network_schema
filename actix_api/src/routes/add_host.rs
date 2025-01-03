use actix_web::{error, post, web, Error, HttpResponse};
use std::collections::HashMap;
use serde_json::json;
use tokio::sync::Mutex;
use std::sync::Arc;
use ipaddress::IPAddress;

use crate::pross::{requester::DataSource, devices::Device};

/// HTTP POST endpoint to add a new host to the host dictionary.
/// 
/// # Arguments
/// 
/// * `data_source_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, DataSource>>>>` representing the host dictionary.
/// * `request` - A `web::Json<AddHostRequest>` representing the request body containing host details.
/// 
/// # Returns
/// 
/// An `impl Responder` containing an `HttpResponse` indicating the result of the operation.
#[post("/add_host")]
pub async fn add_host(
    data_source_dictionary: web::Data<Arc<Mutex<HashMap<String, DataSource>>>>, 
    request_device: web::Json<Device>
) -> Result<HttpResponse, Error> {
    // Validate the IP address or hostname
    if let Err(_) = validate_host(&request_device.ip) {
        return Err(error::ErrorBadRequest("Host cannot be added"));
    }

    // Lock the host dictionary for writing.
    let mut host_dictionary = data_source_dictionary.lock().await;

    // Insert the new host into the dictionary.
    host_dictionary.insert(request_device.ip.clone(), DataSource::Device(request_device.clone()));
    // Return an HTTP response indicating successful addition.
    Ok(HttpResponse::Ok().json(json!({"message": &format!("{} added successfully", request_device.ip)})))
}

/// Validate if the provided host is a valid IP address or hostname.
fn validate_host(ip: &str) -> Result<(), &'static str> {
    // Check if the host is a valid IP address.
    if IPAddress::parse(ip).is_ok() {
        return Ok(());
    }

    Err("Invalid IP address or hostname.")
}
