use actix_web::{error, get, web, Error, HttpResponse};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::logic::requester::{DataSource, Requester};
use crate::logic::services_builder::connectivity_services_vector_build;

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
    data_source_dictionary: web::Data<Arc<Mutex<HashMap<String, DataSource>>>>,
) -> Result<HttpResponse, Error> {
    let id = id.clone();
    let cloned_data_source_dictionary = data_source_dictionary.clone();
    let cloned_data_source_dictionary = cloned_data_source_dictionary.lock().await;
    // Lock the host dictionary for reading.
    if let Some(data_source) = cloned_data_source_dictionary.get(&id) {
        let services_value = Requester::get_services(data_source)
            .await
            .map_err(|_| error::ErrorNotAcceptable("Cannot extract Services from data_sources"))?;
        let services = connectivity_services_vector_build(&services_value);

        Ok(HttpResponse::Ok().json(json!(services)))
    } else {
        Err(error::ErrorNotFound("Id not on database"))
    }
}
