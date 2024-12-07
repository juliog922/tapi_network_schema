use gloo_net::Error;
use gloo_net::http::Request;
use wasm_bindgen::JsValue;
use web_sys::{File, FormData, Blob, BlobPropertyBag};
use serde_json::{Value, from_str, json};
use serde::{Deserialize, Serialize};

const API_URL: &'static str = "/api";
//const API_URL: &'static str = "http://localhost:8080";

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Device {
    pub ip: String,      // Host name or IP address of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>, // Optional port number
    pub auth: Auth,        // Authentication method (enum)
}

/// Enum representing the different authentication methods
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Auth {
    Oauth2(Oauth2), // OAuth2 Authentication
    BasicAuth(BasicAuth), // Basic Authentication
    Custom(CustomAuth),   // Custom Authentication
}

/// Represents Basic Authentication with username and password
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BasicAuth {
    pub username: String, // Username for authentication
    pub password: String, // Password for authentication
}

/// Represents OAuth2 Authentication with additional fields for grant type and authentication URL
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Oauth2 {
    pub username: String,   // Username for OAuth2 authentication
    pub password: String,   // Password for OAuth2 authentication
    pub grant_type: String, // Grant type for OAuth2 (e.g., client_credentials, password)
    pub auth_sufix: String,   // URL to request OAuth2 token
}

/// Represents Custom Authentication with an arbitrary body and authentication URL
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CustomAuth {
    pub auth_body: Value, // A JSON object containing custom authentication data
    pub auth_sufix: String, // URL for custom authentication
}

/// Fetches JSON schema from the server for a given IP address.
///
/// # Arguments
///
/// * `ip` - The IP address for which the JSON schema is to be fetched.
/// * `service_uuid` - The service_uuid for which the JSON schema is to be fetched.
///
/// # Returns
///
/// * `Result<Value, Error>` - Returns the JSON schema as a `Value` if successful, or an error if the request fails.
pub async fn get_schema(ip: String, service_uuid: String) -> Result<Value, Error> {
    let response = Request::get(&format!("{}/get_schema/{}/{}",API_URL, &ip, &service_uuid))
        .send()
        .await?;
    let text = response.text().await?;
    let json: Value = from_str(&text)?;
    Ok(json)
}

/// Fetches JSON schema from the server for a given IP address.
///
/// # Arguments
///
/// * `ip` - The IP address for which the JSON schema is to be fetched.
///
/// # Returns
///
/// * `Result<Value, Error>` - Returns the JSON schema as a `Value` if successful, or an error if the request fails.
pub async fn get_services(ip: String) -> Result<Value, Error> {
    let response = Request::get(&format!("{}/get_services/{}",API_URL, &ip))
        .send()
        .await?;
    let text = response.text().await?;
    let json: Value = from_str(&text)?;
    Ok(json)
}

/// Adds a new device to the server.
///
/// # Arguments
///
/// * `device` - The device.
///
/// # Returns
///
/// * `Result<Value, String>` - Returns the server's response as a `Value` if successful, or an error message if the request fails.
pub async fn add_device(
    device: Device,
) -> Result<Value, String> {
    if let Ok(builder) = Request::post(&format!("{}/add_host", API_URL))
        .header("Accept", "application/json")
        .json(&json!(device)) {
            if let Ok(response) = builder.send().await {
                if let Ok(text) = response.text().await {
                    if let Ok(json) = from_str(&text) {
                        Ok(json)
                    } else {
                        Err(String::from("Failed to parse response JSON."))
                    }
                } else {
                    Err(String::from("Empty API response. The API might not be available."))
                }
            } else {
                Err(String::from("Request failed. The API might not be available."))
            }
    } else {
        Err(String::from("Failed to create request. Cannot add new device."))
    }
}

/// Fetches the list of devices from the server.
///
/// # Returns
///
/// * `Result<Value, Error>` - Returns the list of devices as a `Value` if successful, or an error if the request fails.
pub async fn get_devices() -> Result<Value, Error> {
    let response = Request::get(&format!("{}/get_hosts", API_URL))
        .send()
        .await?;
    let text = response.text().await?;
    let json: Value = from_str(&text)?;
    Ok(json)
}

/// Deletes a data source from the server.
///
/// # Arguments
///
/// * `id` - The ID of the data source to be deleted.
///
/// # Returns
///
/// * `Result<Value, Error>` - Returns the server's response as a `Value` if successful, or an error if the request fails.
pub async fn delete_device(id: &str) -> Result<Value, Error> {
    let response = Request::delete(&format!("{}/delete_host/{}",API_URL, id))
        .send()
        .await?;
    let text = response.text().await?;
    let json: Value = from_str(&text)?;
    Ok(json)
}

pub async fn upload_connectivity_files(
    topology: Option<File>,
    connections: Option<File>,
    connectivity_services: Option<File>,
    complete_context: Option<File>,
    id: String,
) -> Result<Value, String> {
    // Crear un objeto FormData para enviar los archivos
    let form_data = FormData::new().map_err(|e| e.as_string().unwrap_or("Error al crear FormData".to_string()))?;

    // Agregar los archivos a FormData
    if let Some(context) = complete_context {
        form_data.append_with_blob("complete_context_file", &context).map_err(|e| e.as_string().unwrap_or("Error al agregar el archivo de Complete Context".to_string()))?;
    } else {
        form_data.append_with_blob("topology_file", &topology.unwrap()).map_err(|e| e.as_string().unwrap_or("Error al agregar el archivo de Topology".to_string()))?;
        form_data.append_with_blob("connections_file", &connections.unwrap()).map_err(|e| e.as_string().unwrap_or("Error al agregar el archivo de Connections".to_string()))?;
        form_data.append_with_blob("connectivity_services_file", &connectivity_services.unwrap()).map_err(|e| e.as_string().unwrap_or("Error al agregar el archivo de Connectivity Services".to_string()))?;
    }

    let json_object = json!({ "id": &id });
    let json_string = serde_json::to_string(&json_object).map_err(|e| format!("Error al serializar el JSON: {}", e))?;
    let json_jsvalue = JsValue::from_str(&json_string);
    let json_jsvalue_array = js_sys::Array::from_iter(std::iter::once(json_jsvalue));
    

    let blob_options = BlobPropertyBag::new();
    blob_options.set_type("application/json");

    let json_blob_result = Blob::new_with_str_sequence_and_options(&json_jsvalue_array, &blob_options);
    let json_blob = json_blob_result.unwrap();
    // Agregar el Blob JSON al FormData
    form_data.append_with_blob("json", &json_blob).map_err(|e| e.as_string().unwrap_or("Error al agregar el JSON al FormData".to_string()))?;

    // Construir la solicitud POST con los archivos
    let builder = Request::post(&format!("{}/upload_services", API_URL))
        .header("Accept", "multipart/form-data")
        .body(form_data) // Usamos `body` para adjuntar FormData
        .map_err(|e| format!("Error al crear la solicitud: {}", e))?;

    // Enviar la solicitud y obtener la respuesta
    if let Ok(response) = builder.send().await {
        // Leer el texto de la respuesta
        if let Ok(text) = response.text().await {
            // Parsear la respuesta como JSON
            if let Ok(json) = serde_json::from_str(&text) {
                Ok(json)
            } else {
                Err(String::from("Error al parsear la respuesta JSON."))
            }
        } else {
            Err(String::from("Error al leer la respuesta."))
        }
    } else {
        Err(String::from("Error al enviar la solicitud."))
    }
}