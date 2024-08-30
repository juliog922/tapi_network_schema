use actix_web::{error, web, get, Error, HttpResponse};
use serde_json::{Value, from_str};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;

use crate::HostParameters;
use crate::utils::to_list;
use crate::pross::{
    conn_services::services_vector,
    build_endpoints::endpoints_creation,
    lower_transform::lower_conn_transformation,
};

fn get_json_from_file(file_name: &str) -> Result<Value, Error> {
    let current_dir = env::current_dir()?;

    // Definir la ruta relativa
    let relative_path = Path::new(file_name);

    // Construir la ruta completa
    let file_path = current_dir.join(relative_path);

    // Abre el archivo
    let mut file = File::open(&file_path)?;

    // Lee el contenido del archivo en una cadena
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;

    // Convierte la cadena JSON en un valor `serde_json::Value`
    let v: Value = from_str(&json_str)?;
    Ok(v)
}

/// HTTP GET endpoint to retrieve JSON data for a specified host.
/// 
/// # Arguments
/// 
/// * `host` - A `web::Path<String>` representing the host.
/// * `host_dictionary` - A `web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>` representing the host dictionary.
/// 
/// # Returns
/// 
/// An `Result<HttpResponse, Error>` containing the JSON response.
#[get("/get_json/{host}")]
async fn connectivity_services(
    host: web::Path<String>, 
    host_dictionary: web::Data<Arc<Mutex<HashMap<String, HostParameters>>>>) -> Result<HttpResponse, Error> {

    /*
    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .gzip(true)
        .brotli(true)
        .deflate(true)
        .build()
        .unwrap()
        .get("https://10.95.87.21:18010/restconf/data/tapi-common:context")
        .header("Accept", "application/yang-data+json")
        .header("Accept-Encoding", "gzip, deflate, br")
        .basic_auth("tapi", Some("Zenap_1235!!!"))
        .send()
        .await
        .map_err(|_| error::ErrorNotFound("Request Error"))?;

    let json: Value = response.json().await.map_err(|_| error::ErrorNotFound("Empty Response."))?;
     */

    let host = host.clone();

    // Lock the host dictionary for reading.
    if let Ok(new_host_dictionary) = host_dictionary.lock() {
        if let Some(current_host) = new_host_dictionary.clone().get(&host) {
            let _host_parameters = current_host.clone();
        } else {
            return Err(error::ErrorNotFound("Host not on database"));
        }
    }

    let connectivity_services = to_list(get_json_from_file("connectivity_services.json")?)?;
    let connections = to_list(get_json_from_file("connections.json")?)?;
    let _services_interface_point = to_list(get_json_from_file("services_interface_point.json")?)?;
    let topology = get_json_from_file("topology.json")?;

    let mut schema = services_vector(&connectivity_services)?;
    endpoints_creation(&topology, &connections, &mut schema)?;
    lower_conn_transformation(&mut schema)?;

    Ok(HttpResponse::Ok().json(schema))
}