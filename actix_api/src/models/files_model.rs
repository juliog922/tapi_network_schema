use actix_multipart::form::{json::Json, tempfile::TempFile, MultipartForm};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::logic::file_handler::get_file_path;

/// Metadata associated with the uploaded files, including an identifier.
#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub id: String,
}

/// Represents the form used for uploading files via multipart requests.
#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub json: Json<Metadata>,
    #[multipart(limit = "300MB")]
    pub topology_file: Option<TempFile>,
    #[multipart(limit = "300MB")]
    pub connections_file: Option<TempFile>,
    #[multipart(limit = "300MB")]
    pub connectivity_services_file: Option<TempFile>,
    #[multipart(limit = "300MB")]
    pub complete_context_file: Option<TempFile>,
}

/// Represents a complete context file upload.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Complete {
    pub id: String,
    pub complete_context_path: String,
}

/// Represents files uploaded in parts (topology, connections, and connectivity services).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ByPart {
    pub id: String,
    pub topology_path: String,
    pub connections_path: String,
    pub connectivity_services_path: String,
}

/// Enum representing either a complete file upload or a by-part upload.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesEnum {
    Complete(Complete),
    ByPart(ByPart),
}

impl UploadForm {
    /// Converts the upload form into a `FilesEnum` based on the presence of files.
    ///
    /// # Returns
    /// - `FilesEnum::Complete` if a complete context file is provided.
    /// - `FilesEnum::ByPart` if topology, connections, and connectivity services files are provided.
    pub fn to_filesenum(&self) -> Result<FilesEnum, Error> {
        if self.complete_context_file.is_some() {
            Ok(FilesEnum::Complete(Complete::from_uploadform(self)?))
        } else {
            Ok(FilesEnum::ByPart(ByPart::from_uploadform(self)?))
        }
    }
}

impl Complete {
    /// Creates a `Complete` instance from an `UploadForm`.
    ///
    /// # Arguments
    /// - `form`: Reference to the `UploadForm` containing the uploaded complete context file.
    ///
    /// # Returns
    /// A `Complete` instance with the file path resolved.
    pub fn from_uploadform(form: &UploadForm) -> Result<Self, Error> {
        let id = form.json.id.clone();

        Ok(Self {
            id: id.clone(),
            complete_context_path: get_file_path(
                form.complete_context_file.as_ref().unwrap(),
                &id,
                "context",
            )?,
        })
    }
}

impl ByPart {
    /// Creates a `ByPart` instance from an `UploadForm`.
    ///
    /// # Arguments
    /// - `form`: Reference to the `UploadForm` containing the uploaded files in parts.
    ///
    /// # Returns
    /// A `ByPart` instance with the paths of all part files resolved.
    pub fn from_uploadform(form: &UploadForm) -> Result<Self, Error> {
        let id = form.json.id.clone();

        Ok(Self {
            id: id.clone(),
            topology_path: get_file_path(form.topology_file.as_ref().unwrap(), &id, "topology")?,
            connections_path: get_file_path(
                form.connections_file.as_ref().unwrap(),
                &id,
                "connections",
            )?,
            connectivity_services_path: get_file_path(
                form.connectivity_services_file.as_ref().unwrap(),
                &id,
                "connectivity_services",
            )?,
        })
    }
}
