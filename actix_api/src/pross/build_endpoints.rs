use actix_web::{Error, error};
use serde_json::{Number, Value};

use crate::utils::{
    find_value_with_parent_value,
    matching,
};
use crate::models::{
    endpoint::Endpoint,
    node::Node,
};

/// Processes and creates endpoints within the given schema by analyzing topology and connections.
///
/// This function iterates over services, nodes, and endpoints to populate or update endpoint data in
/// the provided schema based on the given topology and connections data.
///
/// # Parameters
/// - `topology_json`: A JSON value representing the topology data. This includes information about node-edge points, links, and other topological details.
/// - `connections_json`: A vector of JSON values representing connection data. This includes details about lower connections and endpoints.
/// - `schema`: A mutable reference to a JSON value representing the schema. This is where new endpoints and nodes will be added or updated.
///
/// # Returns
/// - `Result<(), Error>`: Returns `Ok(())` on success, or an `Error` if something goes wrong, such as missing required fields.
pub fn endpoints_creation(topology_json: &Value, connections_json: &Vec<Value>, schema: &mut Value) -> Result<(), Error> {
    // Check if the schema contains the "connectivity_services" key
    if let Some(connectivity_services) = schema.as_object_mut().unwrap().get_mut(&"connectivity_services".to_string()) {
        
        let services_array: &mut Vec<Value> = connectivity_services.as_array_mut().unwrap();
        let mut service_index: usize = 0;
        
        // Iterate over each service in the services array
        while service_index < services_array.len() {
            
            if let Some(nodes) = services_array[service_index].as_object_mut().unwrap().get_mut(&"nodes".to_string()) {
                
                let nodes_array: &mut Vec<Value> = nodes.as_array_mut().unwrap();
                let mut node_index = 0;
                
                // Iterate over each node in the nodes array
                while node_index < nodes_array.len() {
                    // Collect all node UUIDs for quick lookup
                    let node_uuid_array: Vec<Value> = nodes_array.iter().map(|node| node.as_object().unwrap().get(&"node_uuid".to_string()).unwrap().clone()).collect();
                    let mut new_nodes: Vec<Node> = vec![];

                    let current_node_uuid = nodes_array[node_index].as_object().unwrap().get(&"node_uuid".to_string()).unwrap().clone();

                    if let Some(endpoints) = nodes_array[node_index].as_object_mut().unwrap().get_mut(&"end_points".to_string()) {
                        
                        let endpoints_array: &mut Vec<Value> = endpoints.as_array_mut().unwrap();
                        let mut endpoint_index = 0;
                        
                        // Iterate over each endpoint in the endpoints array
                        while endpoint_index < endpoints_array.len() {

                            let mut endpoint_node_uuid: Vec<(Option<Endpoint>, Option<Value>)> = vec![];

                            let mut option_node_uuid: Option<Value> = None;

                            let mut connection_uuid: Value = Value::String("".to_string());
                            let mut protocol_qualifier: Value = Value::String("".to_string());
                            let mut option_node_edge_point: Option<Value> = None;
                            let mut client_node_edge_point_uuid: Option<Value> = None;
                            let mut lower_connections: Option<Value> = None;
                            let mut link_uuid: Option<Value> = None;
                            let id = endpoints_array[endpoint_index].as_object().unwrap().get(&"id".to_string()).unwrap().clone();

                            // Check if the endpoint has a link UUID and process it
                            if let Some(link_conn) = endpoints_array[endpoint_index].as_object().unwrap().get(&"link_uuid".to_string()) {

                                if let Ok(link_section) = find_value_with_parent_value(
                                    topology_json, 
                                    &link_conn, 
                                    0, 
                                    "uuid") {

                                        if let Some(node_edge_points) = link_section.as_object().unwrap().get(&"node-edge-point".to_string()) {
                                            // Iterate over node edge points to find connections
                                            for node_edge_point in node_edge_points.as_array().unwrap() {
                                                if node_edge_point.as_object().unwrap().get(&"node-uuid".to_string()).unwrap() != &current_node_uuid {

                                                    option_node_uuid = Some(node_edge_point.as_object().unwrap().get(&"node-uuid".to_string()).unwrap().clone());

                                                    let node_edge_point_uuid = node_edge_point.as_object().unwrap().get(&"node-edge-point-uuid".to_string()).unwrap().clone();

                                                    if let Ok(parent_topology) =  find_value_with_parent_value(
                                                        topology_json, 
                                                        &node_edge_point_uuid, 
                                                        1, 
                                                        "parent-node-edge-point"){
                                                            connection_uuid = parent_topology.as_object().unwrap().get(&"uuid".to_string()).unwrap().clone();
                                                            
                                                            // Check connections to find lower connections
                                                            for connection in connections_json {
                                                                if let Ok(node_edge_lower) = find_value_with_parent_value(
                                                                    connection, 
                                                                    &connection_uuid, 
                                                                    1, 
                                                                    "connection-end-point") {
                                                                    if let Some(lower_conn) = node_edge_lower.as_object().unwrap().get(&"lower-connection".to_string()) {
                                                                        lower_connections = Some(lower_conn.clone());
                                                                    }
                                                                }
                                                            }

                                                            // Extract protocol qualifier and client node edge point UUID
                                                            if let Some(layer_prot) = parent_topology.as_object().unwrap().get(&"layer-protocol-qualifier".to_string()) {
                                                                protocol_qualifier = layer_prot.clone();
                                                            }
                                                            if let Some(client_nop) = parent_topology.as_object().unwrap().get(&"client-node-edge-point".to_string()) {
                                                                if let Ok(nop) = find_value_with_parent_value(
                                                                    client_nop, 
                                                                    &node_edge_point.as_object().unwrap().get(&"node-uuid".to_string()).unwrap().clone(), 
                                                                    0, 
                                                                    "node-uuid") {
                                                                        client_node_edge_point_uuid = Some(nop.as_object().unwrap().get(&"node-edge-point-uuid".to_string()).unwrap().clone());
                                                                    }
                                                            }
                                                    }

                                                    // Extract link UUID
                                                    if let Ok(link) = find_value_with_parent_value(
                                                        topology_json, 
                                                        &node_edge_point_uuid, 
                                                        1, 
                                                        "node-edge-point") {
                                                            if let Some(uuid) = link.as_object().unwrap().get(&"uuid".to_string()) {
                                                                link_uuid = Some(uuid.clone());
                                                            }
                                                    }

                                                    // Create a new endpoint and add it to the list
                                                    endpoint_node_uuid.push(
                                                        (
                                                            Some(
                                                                Endpoint::new(
                                                                    connection_uuid.clone(),
                                                                    node_edge_point_uuid,
                                                                    protocol_qualifier.clone(),
                                                                    client_node_edge_point_uuid.clone(),
                                                                    None,
                                                                    lower_connections.clone(),
                                                                    link_uuid.clone(),
                                                                    Value::Number(Number::from(id.as_i64().unwrap() + 1)),
                                                                ),
                                                            ),
                                                            option_node_uuid.clone()
                                                        )
                                                    );
                                                }
                                            }
                                        }
                                }
                            }

                            // Process lower connections for the endpoint
                            if let Some(lower_conns) = endpoints_array[endpoint_index].as_object().unwrap().get(&"lower_connections".to_string()) {
                                for lower_conn in lower_conns.as_array().unwrap() {

                                    let connection_uuid = lower_conn.as_object().unwrap().get("connection-uuid").unwrap();

                                    for connection in connections_json {
                                        if let Ok(endpoints) = find_value_with_parent_value(
                                            connection, 
                                            connection_uuid, 
                                            0, 
                                            "uuid") {

                                                // Iterate over lower connection endpoints
                                                for lower_conn_enpoint in endpoints.as_object().unwrap().get(&"connection-end-point".to_string()).unwrap().as_array().unwrap() {

                                                    let node_edge_point_uuid = lower_conn_enpoint.as_object().unwrap().get(&"node-edge-point-uuid".to_string()).unwrap().clone();

                                                    if let Ok(parent_topology) =  find_value_with_parent_value(
                                                        topology_json, 
                                                        &node_edge_point_uuid, 
                                                        1, 
                                                        "parent-node-edge-point"){
                                                            if let Some(layer_prot) = parent_topology.as_object().unwrap().get(&"layer-protocol-qualifier".to_string()) {
                                                                protocol_qualifier = layer_prot.clone();
                                                            }
                                                            if let Some(client_nop) = parent_topology.as_object().unwrap().get(&"client-node-edge-point".to_string()) {
                                                                if let Ok(nop) = find_value_with_parent_value(
                                                                    client_nop, 
                                                                    &current_node_uuid, 
                                                                    0, 
                                                                    "node-uuid") {
                                                                        client_node_edge_point_uuid = Some(nop.as_object().unwrap().get(&"node-edge-point-uuid".to_string()).unwrap().clone());
                                                                    }
                                                            }
                                                    }
                                                    if let Ok(link) = find_value_with_parent_value(
                                                        topology_json, 
                                                        &node_edge_point_uuid, 
                                                        1, 
                                                        "node-edge-point") {
                                                            if let Some(uuid) = link.as_object().unwrap().get(&"uuid".to_string()) {
                                                                link_uuid = Some(uuid.clone());
                                                            }
                                                    }

                                                    // Create a new endpoint and add it to the list
                                                    endpoint_node_uuid.push(
                                                        (
                                                            Some(
                                                                Endpoint::new(
                                                                    connection_uuid.clone(),
                                                                    node_edge_point_uuid,
                                                                    protocol_qualifier.clone(),
                                                                    client_node_edge_point_uuid.clone(),
                                                                    None,
                                                                    None,
                                                                    link_uuid.clone(),
                                                                    Value::Number(Number::from(id.as_i64().unwrap() + 1)),
                                                                ),
                                                            ),
                                                            Some(lower_conn_enpoint.as_object().unwrap().get(&"node-uuid".to_string()).unwrap().clone())
                                                        )
                                                    );
                                                }     
                                        }
                                    }
                                    
                                }
                            } else if let Some(endpoint_nepu) = endpoints_array[endpoint_index].as_object().unwrap().get(&"node_edge_point_uuid".to_string()) {

                                if let Ok(client_endpoint) = find_value_with_parent_value(
                                    topology_json, 
                                    endpoint_nepu, 
                                    1, 
                                    "client-node-edge-point"
                                ) {
                                    connection_uuid = client_endpoint.as_object().unwrap().get(&"uuid".to_string()).unwrap().clone();
                                    protocol_qualifier = client_endpoint.as_object().unwrap().get(&"layer-protocol-qualifier".to_string()).unwrap().clone();

                                    if let Ok(conn_end_point) = find_value_with_parent_value(
                                        &client_endpoint, 
                                        endpoint_nepu, 
                                        0, 
                                        "node-edge-point-uuid"
                                    ) {
                                        client_node_edge_point_uuid = Some(conn_end_point.as_object().unwrap().get(&"node-edge-point-uuid".to_string()).unwrap().clone()); // topology_uuid
                                    }

                                    option_node_edge_point = Some(matching(true, &client_endpoint, "/parent-node-edge-point/node-edge-point-uuid")?); ////////////////////
                                    option_node_uuid = Some(matching(true, &client_endpoint, "/parent-node-edge-point/node-uuid")?) ///////////////////////////
                                }

                                if let Some(node_edge_point) = option_node_edge_point.clone() {
                                    // Check connections and create new endpoints
                                    for connection in connections_json {
                                        if let Ok(node_edge_lower) = find_value_with_parent_value(
                                            connection, 
                                            &connection_uuid, 
                                            1, 
                                            "connection-end-point") {
                                            if let Some(lower_conn) = node_edge_lower.as_object().unwrap().get(&"lower-connection".to_string()) {
                                                lower_connections = Some(lower_conn.clone());
                                            }
                                        }
                                    }
                                    if let Ok(link) = find_value_with_parent_value(
                                        topology_json, 
                                        &node_edge_point, 
                                        1, 
                                        "node-edge-point") {
                                            if let Some(uuid) = link.as_object().unwrap().get(&"uuid".to_string()) {
                                                link_uuid = Some(uuid.clone());
                                            }
                                    }

                                    // Add new endpoint to the list
                                    endpoint_node_uuid.push(
                                        (
                                            Some(
                                                Endpoint::new(
                                                    connection_uuid.clone(),
                                                    node_edge_point.clone(),
                                                    protocol_qualifier.clone(),
                                                    client_node_edge_point_uuid.clone(),
                                                    None,
                                                    lower_connections.clone(),
                                                    link_uuid.clone(),
                                                    Value::Number(Number::from(id.as_i64().unwrap() + 1)),
                                                ),
                                            ),
                                            option_node_uuid.clone()
                                        )
                                    );
                                }
                                
                            }
                            if let Some(lower_connection_uuid) = endpoints_array[endpoint_index].as_object().unwrap().get(&"connection_end_point_uuid".to_string()) {

                                for connection in connections_json {

                                    if let Ok(node_edge_lower) = find_value_with_parent_value(
                                        connection, 
                                        &lower_connection_uuid, 
                                        0, 
                                        "lower-connection") {

                                        if let Some(lower_conn) = node_edge_lower.as_object().unwrap().get(&"lower-connection".to_string()) {
                                            lower_connections = Some(lower_conn.clone());
                                        }

                                        // Find and process client endpoint
                                        if let Ok(connection_end_point) = find_value_with_parent_value(
                                            &node_edge_lower, 
                                            &current_node_uuid, 
                                            0, 
                                            "node-uuid") {

                                                let node_edge_point_uuid = connection_end_point.as_object().unwrap().get(&"node-edge-point-uuid".to_string()).unwrap().clone();
                                                connection_uuid = connection_end_point.as_object().unwrap().get(&"connection-end-point-uuid".to_string()).unwrap().clone();

                                                if let Ok(client_endpoint) = find_value_with_parent_value(
                                                    topology_json, 
                                                    &node_edge_point_uuid, 
                                                    1, 
                                                    "client-node-edge-point"
                                                ) {
                                                    protocol_qualifier = client_endpoint.as_object().unwrap().get(&"layer-protocol-qualifier".to_string()).unwrap().clone();
                
                                                    if let Ok(conn_end_point) = find_value_with_parent_value(
                                                        &client_endpoint, 
                                                        &node_edge_point_uuid, 
                                                        0, 
                                                        "node-edge-point-uuid"
                                                    ) {
                                                        client_node_edge_point_uuid = Some(conn_end_point.as_object().unwrap().get(&"node-edge-point-uuid".to_string()).unwrap().clone());
                                                    }

                                                    option_node_uuid = Some(matching(true, &client_endpoint, "/parent-node-edge-point/node-uuid")?);

                                                    if let Ok(link) = find_value_with_parent_value(
                                                        topology_json, 
                                                        &node_edge_point_uuid, 
                                                        1, 
                                                        "node-edge-point") {
                                                            if let Some(uuid) = link.as_object().unwrap().get(&"uuid".to_string()) {
                                                                link_uuid = Some(uuid.clone());
                                                            }
                                                    }

                                                    // Add new endpoint to the list
                                                    endpoint_node_uuid.push(
                                                        (
                                                            Some(
                                                                Endpoint::new(
                                                                    connection_uuid.clone(),
                                                                    node_edge_point_uuid.clone(),
                                                                    protocol_qualifier.clone(),
                                                                    client_node_edge_point_uuid.clone(),
                                                                    None,
                                                                    lower_connections.clone(),
                                                                    link_uuid.clone(),
                                                                    Value::Number(Number::from(id.as_i64().unwrap() - 1)),
                                                                ),
                                                            ),
                                                            option_node_uuid.clone()
                                                        )
                                                    );
                                                }
                                            }
                                    }
                                }
                            } 
                            if let Some(endpoint_client) = endpoints_array[endpoint_index].as_object().unwrap().get(&"client_node_edge_point_uuid".to_string()) {

                                if let Ok(client_endpoint) = find_value_with_parent_value(
                                    topology_json, 
                                    &endpoint_client, 
                                    1, 
                                    "parent-node-edge-point"
                                ) {
                                    connection_uuid = client_endpoint.as_object().unwrap().get(&"uuid".to_string()).unwrap().clone();
                                    protocol_qualifier = client_endpoint.as_object().unwrap().get(&"layer-protocol-qualifier".to_string()).unwrap().clone();

                                    option_node_edge_point = Some(matching(true, &client_endpoint, "/parent-node-edge-point/node-edge-point-uuid")?);
                                    option_node_uuid = Some(matching(true, &client_endpoint, "/parent-node-edge-point/node-uuid")?)
                                }

                                if let Some(node_edge_point) = option_node_edge_point.clone() {
                                    for connection in connections_json {
                                        if let Ok(node_edge_lower) = find_value_with_parent_value(
                                            connection, 
                                            &connection_uuid, 
                                            1, 
                                            "connection-end-point") {
                                            if let Some(lower_conn) = node_edge_lower.as_object().unwrap().get(&"lower-connection".to_string()) {
                                                lower_connections = Some(lower_conn.clone());
                                            }
                                        }
                                    }
                                    if let Ok(link) = find_value_with_parent_value(
                                        topology_json, 
                                        &node_edge_point, 
                                        1, 
                                        "node-edge-point") {
                                            if let Some(uuid) = link.as_object().unwrap().get(&"uuid".to_string()) {
                                                link_uuid = Some(uuid.clone());
                                            }
                                    }

                                    // Add new endpoint to the list
                                    endpoint_node_uuid.push(
                                        (
                                            Some(
                                                Endpoint::new(
                                                    connection_uuid,
                                                    node_edge_point,
                                                    protocol_qualifier,
                                                    client_node_edge_point_uuid,
                                                    None,
                                                    lower_connections,
                                                    link_uuid,
                                                    Value::Number(Number::from(id.as_i64().unwrap() - 1)),
                                                ),
                                            ),
                                            option_node_uuid
                                        )
                                    );
                                }
                                
                            }
                            
                            // Add or update nodes with new endpoints
                            for (new_endpoint, option_node_uuid) in endpoint_node_uuid {
                                if let Some(add_endpoint) = new_endpoint {
                                    if let Some(node_uuid) =  option_node_uuid {
                                        if !node_uuid_array.contains(&node_uuid) {
                                            if let Value::String(node_uuid_string) =  node_uuid{
                                                new_nodes.push(Node {
                                                    node_uuid: node_uuid_string,
                                                    endpoints: vec![add_endpoint],
                                                });
                                            }
                                        } else {
                                            if !endpoints_array.iter().any(
                                                |e| e.as_object().unwrap().get(&"node_edge_point_uuid".to_string()).unwrap() == &add_endpoint.node_edge_point_uuid
                                            ) && &node_uuid.to_string() == &current_node_uuid.to_string() {
                                                endpoints_array.push(add_endpoint.to_value());
                                            }
                                        }
                                    }
                                }
                            }
                            
                            endpoint_index += 1;
                        }
                        
                    } else {
                        return Err(error::ErrorNotFound("Cannot find end_points on nodes"));
                    }
                    // Add new nodes to the nodes array if they are not already present
                    for new_node in new_nodes {
                        if !nodes_array.iter().any(|n| n.as_object().unwrap().get(&"node_uuid".to_string()).unwrap() == &new_node.node_uuid) {
                            nodes_array.push(new_node.to_value());
                        }
                    }
                    
                    node_index += 1;
                }
                
            } else {
                return Err(error::ErrorNotFound("Cannot find nodes on connectivity_services"));
            }
            service_index += 1;
        }
        
    } else {
        return Err(error::ErrorNotFound("Cannot find connectivity_services on schema"));
    }
    Ok(())
}
