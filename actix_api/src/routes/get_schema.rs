use actix_web::{error, web, get, Error, HttpResponse};
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::pross::{
    requester::{DataSource, Requester},
    schema::build_schema,
    links::link_vector_build,
    connections::connection_vector_build,
    nodes::node_vector_building,
    connectivity_services::Service,
};

/// HTTP GET endpoint to retrieve JSON data for a specified id.
/// 
/// # Arguments
/// 
/// * `id` - A `web::Path<String>` representing the id or ip.
/// * `service_uuid` - A `web::Path<String>` representing the service_uuid.
/// * `data_source_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, DataSource>>>>` representing the data_source dictionary.
/// 
/// # Returns
/// 
/// An `Result<HttpResponse, Error>` containing the JSON response.
#[get("/get_schema/{id}/{service_uuid}")]
async fn schema_by_service(
    path: web::Path<(String, String)>, 
    data_source_dictionary: web::Data<Arc<Mutex<HashMap<String, DataSource>>>>) -> Result<HttpResponse, Error> {
        let (id, service_uuid) = path.into_inner();
        let id = id.clone();
        let cloned_data_source_dictionary = data_source_dictionary.clone();
        let cloned_data_source_dictionary = cloned_data_source_dictionary.lock().await;
        // Lock the host dictionary for reading.
        if let Some(data_source) = cloned_data_source_dictionary.get(&id) {
            let context = Requester::get_service_context(data_source, &service_uuid).await.map_err(|_| error::ErrorNotAcceptable("Cannot extract Services from data_sources"))?;
            let topology = context.topology.clone();
            let connections = context.connections.clone();
            let service_value = context.connectivity_service.clone();

            let link_vector = link_vector_build(&topology);
            let node_vector = node_vector_building(&topology);
            let connection_vector = connection_vector_build(&connections);
            let service = Service::connectivity_service_build(&service_value, &connection_vector);

            let schema = build_schema(&service, &link_vector, &node_vector, &connection_vector).map_err(|_| error::ErrorNotAcceptable("Cannot Build Services from data_sources"))?;

            return Ok(HttpResponse::Ok().json(schema));
        } else {
            return Err(error::ErrorNotFound("Id not on database"));
        }
    }