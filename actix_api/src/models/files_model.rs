use actix_multipart::form::{json::Json, tempfile::TempFile, MultipartForm};
use serde::{Deserialize, Serialize};

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

