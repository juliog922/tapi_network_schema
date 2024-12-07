use actix_web::{error, web, get, Error, HttpResponse};
use serde_json::json;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::pross::{
    requester::{Requester, DataSource},
    connectivity_services::connectivity_services_vector_build,
};

/// HTTP GET endpoint to retrieve JSON data for a specified id.
/// 
/// # Arguments
/// 
/// * `id` - A `web::Path<String>` representing the id or ip.
/// * `data_source_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, DataSource>>>>` representing the data_source dictionary.
/// 
/// # Returns
/// 
/// An `Result<HttpResponse, Error>` containing the JSON response.
#[get("/get_services/{id}")]
async fn connectivity_services(
    id: web::Path<String>, 
    data_source_dictionary: web::Data<Arc<Mutex<HashMap<String, DataSource>>>>) -> Result<HttpResponse, Error> {

        let id = id.clone();
        let data_source_dictionary = data_source_dictionary.lock().await;
        let cloned_data_source_dictionary = data_source_dictionary.clone();
        // Lock the host dictionary for reading.
        if let Some(data_source) = cloned_data_source_dictionary.get(&id) {
            let services_value = Requester::get_services(&data_source).await.map_err(|_| error::ErrorNotAcceptable("Cannot extract Services from data_sources"))?;
            let services = connectivity_services_vector_build(&services_value);

            return Ok(HttpResponse::Ok().json(json!(services)));
        } else {
            return Err(error::ErrorNotFound("Id not on database"));
        }
    }