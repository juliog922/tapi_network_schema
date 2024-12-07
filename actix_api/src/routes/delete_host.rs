use actix_web::{delete, web, Error, HttpResponse};
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::fs;

use crate::pross::{requester::DataSource, files_model::FilesEnum};

/// HTTP DELETE endpoint to remove a host from the host dictionary.
/// 
/// # Arguments
/// 
/// * `data_source_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, DataSource>>>>` representing the host dictionary.
/// * `hostname` - A `web::Path<String>` representing the hostname to be deleted.
/// 
/// # Returns
/// 
/// An `impl Responder` containing an `HttpResponse` indicating the result of the operation.
#[delete("/delete_host/{id}")]
pub async fn delete_host(
    data_source_dictionary: web::Data<Arc<Mutex<HashMap<String, DataSource>>>>,
    id: web::Path<String>
) -> Result<HttpResponse, Error> {
    let id = id.clone();
    // Lock the host dictionary for writing.
    let mut data_source_dictionary = data_source_dictionary.lock().await;
    
    // Attempt to remove the host from the dictionary.
    if let Some(data_source) = data_source_dictionary.get(&id).cloned() {
        match data_source {
            DataSource::Device(_) => {},
            DataSource::FilesEnum(files_enum) => {
                match files_enum {
                    FilesEnum::ByPart(by_part) => {
                        fs::remove_file(&format!("data/{}", by_part.connections_path))?;
                        fs::remove_file(&format!("data/{}", by_part.connectivity_services_path))?;
                        fs::remove_file(&format!("data/{}", by_part.topology_path))?;
                    },
                    FilesEnum::Complete(complete) => {
                        fs::remove_file(&format!("data/{}", complete.complete_context_path))?;
                    },
                }
            },
        }
        data_source_dictionary.remove(&id);
        // Host was successfully removed.
        Ok(HttpResponse::Ok().body(format!("{} removed successfully", id)))
    } else {
        // Host was not found.
        Ok(HttpResponse::NotFound().body(format!("{} not found", id)))
    }
}
