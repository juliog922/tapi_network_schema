use actix_web::{get, web, Responder};
use std::collections::HashMap;
use serde::Serialize;
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::HostParameters;

/// Struct representing the detailed information about a host.
#[derive(Debug, Serialize)]
struct HostInfo {
    host: String,
    port: Option<String>,
    tenant: Option<String>,
    user: String,
}

/// HTTP GET endpoint to retrieve a list of all hosts with their details.
/// 
/// # Arguments
/// 
/// * `host_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>` representing the host dictionary.
/// 
/// # Returns
/// 
/// An `impl Responder` containing the JSON response with a list of hosts and their details.
#[get("/get_hosts")]
pub async fn get_hosts(
    host_dictionary: web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>) 
-> impl Responder {
    // Lock the host dictionary for reading.
    let host_dictionary = host_dictionary.lock().await;
    // Initialize a vector to store host information.
    let mut hosts_info_vector: Vec<HostInfo> = vec![];
    // Collect all host information from the dictionary.
    for (host, parameters) in host_dictionary.iter() {
        let host_info = HostInfo {
            host: host.clone(),
            port: parameters.port.clone(),
            tenant: parameters.tenant.clone(),
            user: parameters.user.clone(),
        };
        hosts_info_vector.push(host_info);
    }
    // Return the JSON response with the list of hosts and their details.
    web::Json(hosts_info_vector)
}