use crate::models::files_model::{ByPart, Complete, UploadForm, FilesEnum};
use crate::logic::file_handler::get_file_path;
use crate::AppError;

impl UploadForm {
    /// Converts the upload form into a `FilesEnum` based on the presence of files.
    ///
    /// # Returns
    /// - `FilesEnum::Complete` if a complete context file is provided.
    /// - `FilesEnum::ByPart` if topology, connections, and connectivity services files are provided.
    pub fn to_filesenum(&self) -> Result<FilesEnum, AppError> {
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
    pub fn from_uploadform(form: &UploadForm) -> Result<Self, AppError> {
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
    pub fn from_uploadform(form: &UploadForm) -> Result<Self, AppError> {
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
