use actix_web::{error, post, web, Error, HttpResponse};
use ipaddress::IPAddress;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::handlers::requester::DataSource;
use crate::models::devices::Device;

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
    request_device: web::Json<Device>,
) -> Result<HttpResponse, Error> {
    // Validate the IP address or hostname
    if validate_host(&request_device.ip).is_err() {
        log::error!("Host cannot be added: {}", &request_device.ip);
        return Err(error::ErrorBadRequest("Host cannot be added"));
    }

    // Lock the host dictionary for writing.
    let mut host_dictionary = data_source_dictionary.lock().await;

    // Insert the new host into the dictionary.
    host_dictionary.insert(
        request_device.ip.clone(),
        DataSource::Device(request_device.clone()),
    );
    // Return an HTTP response indicating successful addition.
    Ok(HttpResponse::Ok()
        .json(json!({"message": &format!("{} added successfully", request_device.ip)})))
}

/// Validates whether the provided string is a valid IP address.
///
/// # Arguments
/// - `ip`: A string slice representing the IP address or hostname to validate.
///
/// # Returns
/// - `Ok(())`: If the IP address is valid.
/// - `Err(&'static str)`: If the IP address or hostname is invalid.
fn validate_host(ip: &str) -> Result<(), &'static str> {
    // Check if the host is a valid IP address.
    if IPAddress::parse(ip).is_ok() {
        return Ok(());
    }

    Err("Invalid IP address or hostname.")
}
