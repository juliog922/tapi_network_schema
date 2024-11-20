use std::collections::HashMap;

use reqwest::Client;
use actix_web::{error, Error};
use serde_json::{Map, Value};

/// Sends a GET request to the provided URL and returns the JSON response.
///
/// # Arguments
///
/// * `url` - The URL to send the GET request to.
/// * `token` - The authentication token for the API.
///
/// # Returns
///
/// * `Ok(Value)` - The parsed JSON response on success.
/// * `Err(Error)` - An error if the request or parsing fails.
///
/// # Errors
///
/// * `ErrorInternalServerError` if the client build fails.
/// * `ErrorNotFound` if the request fails or if the response is not valid JSON.
async fn get_json(url: &str, token: &str) -> Result<Value, Error> {
    Client::builder()
        .danger_accept_invalid_certs(true) // Accept invalid certificates (for dev purposes).
        .gzip(true) // Enable gzip encoding.
        .brotli(true) // Enable brotli encoding.
        .deflate(true) // Enable deflate encoding.
        .build()
        .map_err(|_| error::ErrorInternalServerError("Client Build Error"))? // Handle client build error.
        .get(url) // Set up the GET request.
        .bearer_auth(token) // Set the Bearer authentication token.
        .header("Accept-Encoding", "gzip, deflate, br") // Include accepted encodings.
        .send()
        .await
        .map_err(|_| error::ErrorNotFound("Request Error"))? // Handle request sending errors.
        .json()
        .await
        .map_err(|_| error::ErrorNotFound("Json Error")) // Handle JSON parsing errors.
}

/// Obtains an authentication token by sending a POST request to the API.
///
/// # Arguments
///
/// * `host` - The API host.
/// * `port` - The API port.
/// * `username` - The username for authentication.
/// * `password` - The password for authentication.
/// * `tenant` - The tenant name for authentication.
///
/// # Returns
///
/// * `Ok(String)` - The authentication token on success.
/// * `Err(Error)` - An error if the request or parsing fails.
///
/// # Errors
///
/// * `ErrorInternalServerError` if the client build fails.
/// * `ErrorNotFound` if the request fails or if the response does not contain a valid token.
pub async fn get_token(
    host: &str,
    port: &str,
    username: &str,
    password: &str,
    tenant: &str,
) -> Result<String, Error> {
    // Build the JSON body for the POST request.
    let mut json_body = std::collections::HashMap::new();
    json_body.insert("username", &username);
    json_body.insert("password", &password);

    // Send the POST request to obtain the token.
    let response = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|_| error::ErrorInternalServerError("Client Build Error"))?
        .post(format!("https://{}{}/tron/api/v1/tokens", host, port))
        .json(&json_body)
        .send()
        .await
        .map_err(|_| error::ErrorNotFound("Request Error"))?;

    // Parse the JSON response.
    let json: Value = response.json().await.map_err(|_| error::ErrorNotFound("Json Error"))?;
    if let Value::Object(json_object) = json {
        if let Some(token) = json_object.get("token") {
            if let Value::String(token_str) = token {
                return Ok(token_str.clone()); // Return the token.
            }
        }
    }
    Err(error::ErrorNotFound("Token Not Found Error")) // Handle missing token error.
}

/// Retrieves the topology information from the API.
///
/// # Arguments
///
/// * `token` - The authentication token.
/// * `host` - The API host.
/// * `port` - The API port.
///
/// # Returns
///
/// * `Ok(Value)` - The topology information.
/// * `Err(Error)` - An error if the request or parsing fails.
///
/// # Errors
///
/// * `ErrorNotFound` if the request or response parsing fails.
pub async fn get_topology(token: &str, host: &str, port: &str) -> Result<Value, Error> {
    // Build the URL to fetch topology information.
    let url = format!(
        "https://{}{}/restconf/data/tapi-common:context/tapi-topology:topology-context?fields=topology(uuid)",
        host, port
    );
    
    // Send the request and parse the JSON response.
    let json = get_json(&url, token).await?;

    // Parse the topology context and topology UUID.
    let topology_context = json
        .as_object()
        .ok_or(error::ErrorNotFound("Invalid JSON"))?
        .get("tapi-topology:topology-context")
        .ok_or(error::ErrorNotFound("Topology Context Not Found"))?;
    let topology = topology_context
        .as_object()
        .ok_or(error::ErrorNotFound("Invalid Topology Context"))?
        .get("topology")
        .ok_or(error::ErrorNotFound("Topology Not Found"))?;
    let topologies = topology.as_array().ok_or(error::ErrorNotFound("Invalid Topology"))?;

    // Ensure there is exactly one topology UUID.
    if topologies.len() != 1 {
        return Err(error::ErrorNotFound("There is more or less than one topology uuid"));
    }
    let topology_uuid = topologies[0]
        .as_object()
        .ok_or(error::ErrorNotFound("Invalid Topology Object"))?
        .get("uuid")
        .ok_or(error::ErrorNotFound("UUID Not Found"))?
        .as_str()
        .ok_or(error::ErrorNotFound("Invalid UUID"))?;

    // Fetch links and nodes information using the topology UUID.
    let link_url = format!(
        "https://{}{}/tapi/data/tapi-common:context/tapi-topology:topology-context/topology={}/link",
        host, port, topology_uuid
    );
    let link_json = get_json(&link_url, token).await?;

    let nodes_url = format!(
        "https://{}{}/tapi/data/tapi-common:context/tapi-topology:topology-context/topology={}/node",
        host, port, topology_uuid
    );
    let nodes_json = get_json(&nodes_url, token).await?;

    // Combine links and nodes into a single JSON value.
    let mut topology_hashmap: Map<String, Value> = Map::new();
    topology_hashmap.insert("link".to_string(), link_json.get("tapi-topology:link").unwrap().clone()); 
    topology_hashmap.insert("node".to_string(), nodes_json.get("tapi-topology:node").unwrap().clone()); 
    let topology_object: Value = Value::Object(topology_hashmap);
    let topology_vector: Value = Value::Array(vec![topology_object]);

    Ok(topology_vector) // Return the combined topology information.
}

/// Retrieves the connectivity connections from the API.
///
/// # Arguments
///
/// * `token` - The authentication token.
/// * `host` - The API host.
/// * `port` - The API port.
///
/// # Returns
///
/// * `Ok(Value)` - The connections information.
/// * `Err(Error)` - An error if the request or parsing fails.
///
/// # Errors
///
/// * `ErrorNotFound` if the request or response parsing fails.
pub async fn get_connections(token: &str, host: &str, port: &str) -> Result<Value, Error> {
    let url = format!(
        "https://{}{}/restconf/data/tapi-common:context/tapi-connectivity:connectivity-context",
        host, port
    );
    let json = get_json(&url, token).await?;
    let connections = json
        .as_object()
        .ok_or(error::ErrorNotFound("Invalid JSON"))?
        .get("tapi-connectivity:connection")
        .ok_or(error::ErrorNotFound("Connections Not Found"))?;

    Ok(connections.clone()) // Return the connections information.
}

/// Retrieves connectivity service information for all services.
///
/// # Arguments
///
/// * `token` - The authentication token.
/// * `host` - The API host.
/// * `port` - The API port.
///
/// # Returns
///
/// * `Ok(Value)` - A list of connectivity services.
/// * `Err(Error)` - An error if the request or parsing fails.
///
/// # Errors
///
/// * `ErrorNotFound` if the request or response parsing fails.
pub async fn get_services(token: &str, host: &str, port: &str) -> Result<Value, Error> {
    let url = format!(
        "https://{}{}/restconf/data/tapi-common:context/tapi-connectivity:connectivity-context?fields=connectivity-service(uuid)",
        host, port
    );
    let json = get_json(&url, token).await?;

    let connectivity_context = json
        .as_object()
        .ok_or(error::ErrorNotFound("Invalid JSON"))?
        .get("tapi-connectivity:connectivity-context")
        .ok_or(error::ErrorNotFound("Connectivity Context Not Found"))?;
    let connectivity_services = connectivity_context
        .as_object()
        .ok_or(error::ErrorNotFound("Invalid Connectivity Context"))?
        .get("connectivity-service")
        .ok_or(error::ErrorNotFound("Connectivity Service Not Found"))?
        .as_array()
        .ok_or(error::ErrorNotFound("Invalid Connectivity Service"))?;

    let mut services_vector: Vec<Value> = vec![];
    for service in connectivity_services {
        let service_uuid = service
            .as_object()
            .ok_or(error::ErrorNotFound("Invalid Service Object"))?
            .get("uuid")
            .ok_or(error::ErrorNotFound("UUID Not Found"))?
            .as_str()
            .ok_or(error::ErrorNotFound("Invalid UUID"))?;

        let service_url = format!(
            "https://{}{}/restconf/data/tapi-common:context/tapi-connectivity:connectivity-context/connectivity-service={}",
            host, port, service_uuid
        );
        let service_json = get_json(&service_url, token).await?;
        let service_data = service_json
            .as_object()
            .ok_or(error::ErrorNotFound("Invalid Service JSON"))?
            .get("tapi-connectivity:connectivity-service")
            .ok_or(error::ErrorNotFound("Service Data Not Found"))?;
        services_vector.push(service_data.clone()); // Add the service data to the vector.
    }

    Ok(Value::Array(services_vector)) // Return the list of services.
}
