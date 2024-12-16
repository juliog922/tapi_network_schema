use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::error::Error;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::fs::File;
use actix_multipart::form::{MultipartForm, tempfile::TempFile, json::Json};

/// Función para leer un archivo de TempFile y deserializarlo como JSON
fn read_json_from_file(file: &TempFile) -> io::Result<Value> {
    let mut file_content = String::new();
    file.file.as_file().read_to_string(&mut file_content)?;
    let json: Value = serde_json::from_str(&file_content)?;
    Ok(json)
}

fn get_file_path(complete_context_file: &TempFile, id: &String, file_name: &str) -> Result<String, Error> {
    match read_json_from_file(complete_context_file) {
        Ok(value) => {
            let base_path = Path::new("data");
            let file_path = base_path.join(format!("{}_{}.json", id, file_name));
            let file = File::create(&file_path).map_err(|err| Error::from(format!("File cannot be created: {}", err).as_str()))?;
            serde_json::to_writer_pretty(file, &value).map_err(|err| Error::from(format!("File cannot be writed: {}", err).as_str()))?;

            return Ok(file_path.to_string_lossy().to_string());
        },
        Err(err) => {
            return Err(Error::from(format!("File cannot be readed: {}", err).as_str()));
        }
    };
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub id: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub json: Json<Metadata>,
    #[multipart(limit = "300MB")]
    pub topology_file: Option<TempFile>,             // Archivo para topología
    #[multipart(limit = "300MB")]
    pub connections_file: Option<TempFile>,          // Archivo para conexiones
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
            complete_context_path: get_file_path(form.complete_context_file.as_ref().unwrap(), &id, "context")?,
        })   
    }
}

impl ByPart {
    pub fn from_uploadform(form: &UploadForm) -> Result<Self, Error> {

        let id = form.json.id.clone();

        Ok(Self {
            id: id.clone(),
            topology_path: get_file_path(form.topology_file.as_ref().unwrap(), &id, "topology")?,
            connections_path: get_file_path(form.connections_file.as_ref().unwrap(), &id, "connections")?,
            connectivity_services_path: get_file_path(form.connectivity_services_file.as_ref().unwrap(), &id, "connectivity_services")?,
        })
    }
}