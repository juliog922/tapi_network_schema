use actix_web::{delete, web, Error, HttpResponse};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::HostParameters;

/// HTTP DELETE endpoint to remove a host from the host dictionary.
/// 
/// # Arguments
/// 
/// * `host_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>` representing the host dictionary.
/// * `hostname` - A `web::Path<String>` representing the hostname to be deleted.
/// 
/// # Returns
/// 
/// An `impl Responder` containing an `HttpResponse` indicating the result of the operation.
#[delete("/delete_host/{hostname}")]
pub async fn delete_host(
    host_dictionary: web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>,
    hostname: web::Path<String>
) -> Result<HttpResponse, Error> {
    let hostname = hostname.clone();
    // Lock the host dictionary for writing.
    let mut host_dictionary = host_dictionary.lock().unwrap();
    
    // Attempt to remove the host from the dictionary.
    if host_dictionary.remove(&hostname).is_some() {
        // Host was successfully removed.
        Ok(HttpResponse::Ok().body(format!("{} removed successfully", hostname)))
    } else {
        // Host was not found.
        Ok(HttpResponse::NotFound().body(format!("{} not found", hostname)))
    }
}
