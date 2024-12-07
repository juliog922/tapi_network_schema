use actix_web::{error, web, get, Error, HttpResponse};
use serde_json::Value;
use reqwest::Client;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::fs::File;
use std::io::BufReader;

use crate::DeviceKind;
use crate::utils::{
    to_list, 
    matching,
};
use crate::pross::{
    token_src::{
        get_token,
        get_topology,
        get_connections,
        get_services,
    },
    schema::build_schema,
    links::link_vector_build,
    connections::connection_vector_build,
    nodes::node_vector_building,
    connectivity_services::connectivity_service_vector_build,
};

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
    host_dictionary: web::Data<Arc<Mutex<HashMap<String, DeviceKind>>>>) -> Result<HttpResponse, Error> {

    /*
    let connectivity_services = to_list(get_json_from_file("connectivity_services")?)?;
    let connections = to_list(get_json_from_file("connections.json")?)?;
    let _services_interface_point = to_list(get_json_from_file("services_interface_point.json")?)?;
    let topology = get_json_from_file("topology.json")?;
    */

    let host = host.clone();
    let real_host_dict = host_dictionary.lock().await;
    let cloned_real_host_dict = real_host_dict.clone();
    // Lock the host dictionary for reading.
    if let Some(current_host) = cloned_real_host_dict.get(&host) {

        let connectivity_services: Vec<Value>;
        let connections: Vec<Value>;
        let topology: Value;
        
        match current_host {
            DeviceKind::Files(files_parameters) => {
                if let Some(complete_context_path) = &files_parameters.complete_context_name {
                    let file = File::open(format!("data/{}", complete_context_path))?;
                    let reader = BufReader::new(file);
                    let json_value: Value = serde_json::from_reader(reader)?;

                    connectivity_services = to_list(matching(true, &json_value, "/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")?)?;
                    connections = to_list(matching(true, &json_value, "/tapi-common:context/tapi-connectivity:connectivity-context/connection")?)?;
                    topology = matching(true, &json_value, "/tapi-common:context/tapi-topology:topology-context/topology")?;
                } else {
                    let topology_file = File::open(format!("data/{}", files_parameters.topology_name.clone().unwrap()))?;
                    let topology_reader = BufReader::new(topology_file);
                    topology = serde_json::from_reader(topology_reader)?;

                    let connections_file = File::open(format!("data/{}", files_parameters.connections_name.clone().unwrap()))?;
                    let connections_reader = BufReader::new(connections_file);
                    let connections_value: Value = serde_json::from_reader(connections_reader)?;
                    connections = to_list(connections_value)?;

                    let connectivity_services_file = File::open(format!("data/{}", files_parameters.connectivity_services_name.clone().unwrap()))?;
                    let connectivity_services_reader = BufReader::new(connectivity_services_file);
                    let connectivity_services_value: Value = serde_json::from_reader(connectivity_services_reader)?;
                    connectivity_services = to_list(connectivity_services_value)?;

                }
            },
            DeviceKind::Device(device) => {
                let port = {
                    if let Some(some_port) = device.port.clone() {
                        ":".to_string() + &some_port
                    } else {
                        "".to_string()
                    }
                };

                if let Ok(token) = get_token(&host, &port, &device.user, &device.password).await {

                    connectivity_services = to_list(get_services(&token, &host, &port).await?)?;
                    connections = to_list(get_connections(&token, &host, &port).await?)?;
                    topology = get_topology(&token, &host, &port).await?;
        
                } else {
                    let response = Client::builder()
                    .danger_accept_invalid_certs(true)
                    .gzip(true)
                    .brotli(true)
                    .deflate(true)
                    .build()
                    .unwrap()
                    .get(format!("https://{}{}/restconf/data/tapi-common:context", host, port))
                    .header("Accept", "application/yang-data+json")
                    .header("Accept-Encoding", "gzip, deflate, br")
                    .basic_auth(format!("{}", device.user), Some(format!("{}", device.password)))
                    .send()
                    .await
                    .map_err(|_| error::ErrorNotFound("Request Error"))?;
        
                    let json: Value = response.json().await.map_err(|_| error::ErrorNotFound("Empty Response."))?;
        
                    connectivity_services = to_list(matching(true, &json, "/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")?)?;
                    connections = to_list(matching(true, &json, "/tapi-common:context/tapi-connectivity:connectivity-context/connection")?)?;
                    //let _services_interface_point = to_list(matching(true, &json, "/tapi-common:context/service-interface-point")?)?;
                    topology = matching(true, &json, "/tapi-common:context/tapi-topology:topology-context/topology")?;
                }
            }
        }
        
        let link_vector = link_vector_build(&topology);
        let connection_vector = connection_vector_build(&connections);
        let node_vector = node_vector_building(&topology);
        let service_vector = connectivity_service_vector_build(&connectivity_services, &connection_vector);

        let schema = build_schema(&service_vector, &link_vector, &node_vector, &connection_vector)?; 
        
        
        return Ok(HttpResponse::Ok().json(schema));

    } else {
        return Err(error::ErrorNotFound("Host not on database"));
    }

 
}