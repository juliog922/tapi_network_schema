use actix_web::{error, web, get, Error, HttpResponse};
use serde_json::{Value, from_str};
use reqwest::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;

use crate::HostParameters;
use crate::utils::{to_list, matching};
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
    host = 10.95.87.21
    port = 18010
    user = tapi
    password = Zenap_1235!!!
    let connectivity_services = to_list(get_json_from_file("connectivity_services")?)?;
    let connections = to_list(get_json_from_file("connections.json")?)?;
    let _services_interface_point = to_list(get_json_from_file("services_interface_point.json")?)?;
    let topology = get_json_from_file("topology.json")?;
    */

    let host = host.clone();

    // Lock the host dictionary for reading.
    if let Ok(new_host_dictionary) = host_dictionary.lock() {
        if let Some(current_host) = new_host_dictionary.clone().get(&host) {
            let _host_parameters = current_host.clone();

            let response = Client::builder()
                .danger_accept_invalid_certs(true)
                .gzip(true)
                .brotli(true)
                .deflate(true)
                .build()
                .unwrap()
                .get(format!("https://{}:{}/restconf/data/tapi-common:context", host, current_host.port))
                .header("Accept", "application/yang-data+json")
                .header("Accept-Encoding", "gzip, deflate, br")
                .basic_auth(format!("{}", current_host.user), Some(format!("{}", current_host.password)))
                .send()
                .await
                .map_err(|_| error::ErrorNotFound("Request Error"))?;

            let json: Value = response.json().await.map_err(|_| error::ErrorNotFound("Empty Response."))?;

            let connectivity_services = to_list(matching(true, &json, "/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")?)?;
            let connections = to_list(matching(true, &json, "/tapi-common:context/tapi-connectivity:connectivity-context/connection")?)?;
            let _services_interface_point = to_list(matching(true, &json, "/tapi-common:context/service-interface-point")?)?;
            let topology = matching(true, &json, "/tapi-common:context/tapi-topology:topology-context/topology")?;
            

            let mut schema = services_vector(&connectivity_services)?;
            endpoints_creation(&topology, &connections, &mut schema)?;
            lower_conn_transformation(&mut schema)?;

            return Ok(HttpResponse::Ok().json(schema));

        } else {
            return Err(error::ErrorNotFound("Host not on database"));
        }
    } else {
        return Err(error::ErrorNotFound("Host not on database"));
    }
 
}