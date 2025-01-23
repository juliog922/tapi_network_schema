use crate::error::Error;
use actix_multipart::form::tempfile::TempFile;
use serde_json::Value;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Reads a `TempFile` and deserializes its content into a JSON `Value`.
///
/// # Arguments
///
/// * `file` - Reference to the `TempFile` to be read.
///
/// # Returns
///
/// An `io::Result` containing the JSON `Value` or an I/O error.
fn read_json_from_file(file: &TempFile) -> io::Result<Value> {
    let mut file_content = String::new();
    // Read file content into a string
    file.file.as_file().read_to_string(&mut file_content)?;
    // Deserialize the string as JSON
    let json: Value = serde_json::from_str(&file_content)?;
    Ok(json)
}

/// Generates a file path, writes JSON data from a `TempFile` to a new file, and saves it.
///
/// # Arguments
///
/// * `complete_context_file` - Reference to the input `TempFile` containing JSON data.
/// * `id` - Reference to a string identifier to include in the file name.
/// * `file_name` - File name (without path or extension) to use.
///
/// # Returns
///
/// A `Result` containing the path to the written file or an error.
pub fn get_file_path(
    complete_context_file: &TempFile,
    id: &String,
    file_name: &str,
) -> Result<String, Error> {
    match read_json_from_file(complete_context_file) {
        Ok(value) => {
            let base_path = Path::new("data");
            // Construct the output file path
            let file_path = base_path.join(format!("{}_{}.json", id, file_name));
            // Create the output file
            let file = File::create(&file_path)
                .map_err(|err| Error::from(format!("File cannot be created: {}", err).as_str()))?;
            // Write the JSON data to the file
            serde_json::to_writer_pretty(file, &value)
                .map_err(|err| Error::from(format!("File cannot be writed: {}", err).as_str()))?;

            return Ok(file_path.to_string_lossy().to_string());
        }
        Err(err) => {
            return Err(Error::from(
                format!("File cannot be readed: {}", err).as_str(),
            ));
        }
    };
}
