use actix_multipart::form::{json::Json, tempfile::TempFile, MultipartForm};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::logic::file_handler::get_file_path;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub id: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub json: Json<Metadata>,
    #[multipart(limit = "300MB")]
    pub topology_file: Option<TempFile>, // Archivo para topología
    #[multipart(limit = "300MB")]
    pub connections_file: Option<TempFile>, // Archivo para conexiones
    #[multipart(limit = "300MB")]
    pub connectivity_services_file: Option<TempFile>, // Archivo para servicios de conectividad
    #[multipart(limit = "300MB")]
    pub complete_context_file: Option<TempFile>, // Archivo con el contexto completo
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Complete {
    pub id: String,
    pub complete_context_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ByPart {
    pub id: String,
    pub topology_path: String,
    pub connections_path: String,
    pub connectivity_services_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilesEnum {
    Complete(Complete),
    ByPart(ByPart),
}

impl UploadForm {
    pub fn to_filesenum(&self) -> Result<FilesEnum, Error> {
        if self.complete_context_file.is_some() {
            return Ok(FilesEnum::Complete(Complete::from_uploadform(&self)?));
        } else {
            return Ok(FilesEnum::ByPart(ByPart::from_uploadform(&self)?));
        }
    }
}

impl Complete {
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
