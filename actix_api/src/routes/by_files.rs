use actix_multipart::form::MultipartForm;
use actix_web::{error, post, web, Error, HttpResponse};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::logic::requester::DataSource;
use crate::models::files_model::UploadForm;

/// Handles the `/upload_services` endpoint for uploading service data.
///
/// # Arguments
/// - `data_source_dictionary`: A thread-safe shared dictionary to store the data source.
/// - `MultipartForm(form)`: The multipart form containing the uploaded files and metadata.
///
/// # Returns
/// - `HttpResponse`: A success message if the files are processed and added to the data source.
#[post("/upload_services")]
pub async fn upload_services(
    data_source_dictionary: web::Data<Arc<Mutex<HashMap<String, DataSource>>>>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<HttpResponse, Error> {
    let mut data_source_dictionary = data_source_dictionary.lock().await;

    data_source_dictionary.insert(
        form.json.id.to_string(),
        DataSource::FilesEnum(
            form.to_filesenum()
                .map_err(|_| error::ErrorNotAcceptable("Cannot parse FilesEnum"))?,
        ),
    );

    Ok(
        HttpResponse::Ok()
            .json(json!({"message": &format!("{} added successfully", form.json.id)})),
    )
}
