use actix_web::{error, web, get, Error, HttpResponse};
use serde_json::{Value, from_str};
use reqwest::Client;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::HostParameters;
use crate::utils::{
    to_list, 
    matching,
};
use crate::pross::{
    conn_services::services_vector,
    build_endpoints::endpoints_creation,
    lower_transform::lower_conn_transformation,
    inventory_id::inventory_creation,
    data_fill::link_mapping,
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
    let real_host_dict = host_dictionary.lock().await.clone();
    // Lock the host dictionary for reading.
    if let Some(current_host) = real_host_dict.get(&host) {
        let _host_parameters = current_host.clone();

        let port = {
            if let Some(some_port) = current_host.port.clone() {
                ":".to_string() + &some_port
            } else {
                "".to_string()
            }
        };

        let connectivity_services: Vec<Value>;
        let connections: Vec<Value>;
        let topology: Value;

        if let Ok(token) = get_token(&host, &port, &current_host.user, &current_host.password, &current_host.tenant.clone().unwrap_or_default()).await {

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
            .basic_auth(format!("{}", current_host.user), Some(format!("{}", current_host.password)))
            .send()
            .await
            .map_err(|_| error::ErrorNotFound("Request Error"))?;

            let json: Value = response.json().await.map_err(|_| error::ErrorNotFound("Empty Response."))?;

            connectivity_services = to_list(matching(true, &json, "/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service")?)?;
            connections = to_list(matching(true, &json, "/tapi-common:context/tapi-connectivity:connectivity-context/connection")?)?;
            //let _services_interface_point = to_list(matching(true, &json, "/tapi-common:context/service-interface-point")?)?;
            topology = matching(true, &json, "/tapi-common:context/tapi-topology:topology-context/topology")?;
        }

        /* 
        let link_vector = link_vector_build(&topology);
        let connection_vector = connection_vector_build(&connections);
        let node_vector = node_vector_building(&topology);
        let service_vector = connectivity_service_vector_build(&connectivity_services);

        let schema = build_schema(&service_vector, &link_vector, &node_vector, &connection_vector)?;
        */

        let mut lower_connections_hashmap: HashMap<String, Value> = HashMap::new();
        let mut connection_uuid_hashmap: HashMap<String, Value> = HashMap::new();
        let mut connection_uuid_lower_hashmap: HashMap<String, Value> = HashMap::new();
        let mut nepu_by_connection: HashMap<String, Vec<Value>> = HashMap::new();

        for connection in &connections {
            if let Some(connection_uuid) = connection.get("uuid") {

                let connection_end_point_list = connection.get("connection-end-point").unwrap_or(&Value::Array(vec![])).as_array().unwrap_or(&vec![]).clone();

                nepu_by_connection.insert(
                    connection_uuid.clone().to_string(),
                    connection_end_point_list.clone()
                );
                
                for connection_end_point in connection_end_point_list.iter() {

                    if let Some(lower_connections) = connection.get("lower-connection") {

                        lower_connections_hashmap.insert(
                            connection_end_point.get("node-edge-point-uuid").unwrap().to_string(),
                            lower_connections.clone()
                        );

                        connection_uuid_hashmap.insert(
                            connection_end_point.get("node-edge-point-uuid").unwrap().to_string(),
                            connection_uuid.clone()
                        );

                    } else {

                        connection_uuid_lower_hashmap.insert(
                            connection_end_point.get("node-edge-point-uuid").unwrap().to_string(),
                            connection_uuid.clone()
                        );

                    }
                }

            }
        }

        let link_nepu_hashmap: HashMap<String, Value> = link_mapping(&topology);

        let mut schema = services_vector(&connectivity_services, &topology, 
            &lower_connections_hashmap, &connection_uuid_hashmap, &connection_uuid_lower_hashmap, &link_nepu_hashmap)?;

        endpoints_creation(&topology, &connections, &mut schema, 
            &lower_connections_hashmap, &connection_uuid_hashmap, &connection_uuid_lower_hashmap, &nepu_by_connection , &link_nepu_hashmap)?;

        lower_conn_transformation(&mut schema, &connection_uuid_lower_hashmap)?;
        let schema = inventory_creation(&mut schema)?; 
        
        
        return Ok(HttpResponse::Ok().json(schema));

    } else {
        return Err(error::ErrorNotFound("Host not on database"));
    }

 
}