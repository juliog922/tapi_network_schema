use actix_web::{get, web, Responder};
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::pross::requester::DataSource;


/// HTTP GET endpoint to retrieve a list of all hosts with their details.
/// 
/// # Arguments
/// 
/// * `data_source_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, DataSource>>>>` representing the host dictionary.
/// 
/// # Returns
/// 
/// An `impl Responder` containing the JSON response with a list of hosts and their details.
#[get("/get_hosts")]
pub async fn get_hosts(
    data_source_dictionary: web::Data<Arc<Mutex<HashMap<String, DataSource>>>>) 
-> impl Responder {
    // Lock the host dictionary for reading.
    let data_source_dictionary = data_source_dictionary.lock().await;
    // Initialize a vector to store host information.
    let mut data_source_vector: Vec<DataSource> = vec![];
    // Collect all host information from the dictionary.
    for (_, data_source) in data_source_dictionary.iter() {
        data_source_vector.push(data_source.clone());
    }
    // Return the JSON response with the list of hosts and their details.
    web::Json(data_source_vector)
}