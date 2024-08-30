use actix_web::Error;
use serde_json::{Value, Map, Number};

use crate::utils::{matching, to_list};
use crate::models::{endpoint::Endpoint, service::Service, node::Node};

/// Builds a schema from the provided services JSON vector.
///
/// This function processes a vector of service JSON objects, constructs `Service` and `Node` instances
/// from the provided data, and organizes them into a schema represented as a JSON object.
///
/// # Arguments
///
/// * `services_json` - A vector of JSON values representing the services.
///
/// # Returns
///
/// * `Result<Value, Error>` - A `Result` containing the constructed schema as a `Value` or an `Error` if something goes wrong.
pub fn services_vector(services_json: &Vec<Value>) -> Result<Value, Error> {
    // Create the root schema object
    let mut schema = Value::Object(Map::new());
    
    // Vector to hold the JSON representation of services
    let mut services = vec![];

    // Iterate over each service in the input JSON vector
    for service_item in services_json {
        // Extract the UUID of the service
        let uuid = matching(false, service_item, "uuid")?;
        
        // Initialize a new Service instance with the extracted UUID
        let mut service = Service::new(uuid, None);

        // Track UUIDs of nodes to avoid duplicates
        let mut nodes_uuids = vec![];
        
        // Iterate over each endpoint in the service
        for endpoint in to_list(matching(false, service_item, "end-point")?)? {
            // Extract optional service interface point UUID
            let service_interface_uuid = matching(true, &endpoint, "/service-interface-point/service-interface-point-uuid").ok();
            
            // Iterate over each connection end point in the endpoint
            for conn_end_point in to_list(matching(false, &endpoint, "connection-end-point")?)? {
                // Extract necessary details for each connection end point
                let connection_end_point_uuid = matching(false, &conn_end_point, "connection-end-point-uuid")?;
                let node_edge_point_uuid = matching(false, &conn_end_point, "node-edge-point-uuid")?;
                let layer_protocol_qualifier = matching(false, &endpoint, "layer-protocol-qualifier")?;
                let id = Value::Number(Number::from(1));

                // Extract optional fields
                let lower_connections: Option<Value> = matching(false, &endpoint, "lower-connections").ok();
                let link_uuid = matching(false, &endpoint, "link-uuid").ok();
                
                // Extract the node UUID from the connection end point
                let node_uuid = matching(false, &conn_end_point, "node-uuid")?;
                let node_uuid_str = node_uuid.as_str().unwrap().to_string();

                // Create a new Endpoint instance
                let endpoint = Endpoint::new(
                    connection_end_point_uuid,
                    node_edge_point_uuid,
                    layer_protocol_qualifier,
                    None,
                    service_interface_uuid.clone(),
                    lower_connections,
                    link_uuid,
                    id,
                );

                // Add or update Node instances
                if !nodes_uuids.contains(&node_uuid_str) {
                    // Create a new Node if it does not already exist
                    let mut node = Node::new(node_uuid_str.clone());
                    node.endpoints.push(endpoint);
                    service.add_node(node);
                    nodes_uuids.push(node_uuid_str);
                } else {
                    // Update existing Node by adding the endpoint if it is not already present
                    if let Some(node) = service.nodes.iter_mut().find(|n| n.node_uuid == node_uuid_str) {
                        let is_endpoint_present = node.endpoints.iter().any(|ep| ep.connection_end_point_uuid == endpoint.connection_end_point_uuid);
                        if !is_endpoint_present {
                            node.endpoints.push(endpoint);
                        }
                    }
                }
            }
            
            // Extract and set the service name if available
            for name in to_list(matching(false, service_item, "name")?)? {
                if let Value::String(value_name) = matching(false, &name, "value-name")? {
                    if value_name == "SERVICE_NAME" {
                        service.name = Some(matching(false, &name, "value")?);
                    }
                }
            }
        }

        // Add the service's JSON representation to the services vector
        services.push(service.to_value());
    }

    // Insert the services vector into the root schema object
    schema.as_object_mut().unwrap().insert("connectivity_services".to_string(), Value::Array(services));
    
    // Return the final schema as a Result
    Ok(schema)
}
