use actix_web::{Error, error};
use serde_json::{Value, Number};

/// Transforms the `schema` by modifying the `nodes` and `end_points` within `connectivity_services`.
///
/// This function performs several key operations:
/// 1. It iterates over all `connectivity_services` in the given `schema`.
/// 2. For each service, it processes the `nodes` array by adding a `node_id` field to each node.
/// 3. It also transforms `end_points` within each node, replacing `lower_connections` with individual fields for each connection.
/// 4. Finally, it sorts `nodes` and `end_points` based on their respective IDs.
///
/// # Arguments
///
/// * `schema` - A mutable reference to a `serde_json::Value` representing the schema to be transformed.
///
/// # Returns
///
/// * `Result<(), Error>` - Returns `Ok(())` if the transformation is successful, or an `Error` if any required fields are missing.
pub fn lower_conn_transformation(schema: &mut Value) -> Result<(), Error> {
    // Check if the schema contains the "connectivity_services" key.
    if let Some(connectivity_services) = schema.as_object_mut().unwrap().get_mut(&"connectivity_services".to_string()) {

        // Retrieve the array of services.
        let services_array: &mut Vec<Value> = connectivity_services.as_array_mut().unwrap();

        // Iterate over each service in the array.
        for service in services_array {

            // Check if the service contains the "nodes" key.
            if let Some(nodes) = service.as_object_mut().unwrap().get_mut(&"nodes".to_string()) {

                // Retrieve the array of nodes.
                let nodes_array: &mut Vec<Value> = nodes.as_array_mut().unwrap();
                
                // Initialize node IDs with starting values.
                let mut node_ids: Vec<i64> = vec![1, nodes_array.len() as i64];
                let mut node_ids_index: usize = 0;

                // Iterate over each node.
                for node in nodes_array {

                    // Add a "node_id" field to the node.
                    node.as_object_mut().unwrap().insert("node_id".to_string(), Value::Number(Number::from(node_ids[node_ids_index])));

                    // Toggle between node ID values.
                    if node_ids_index == 0 {
                        node_ids[node_ids_index] += 1;
                        node_ids_index = 1;
                    } else {
                        node_ids[node_ids_index] -= 1;
                        node_ids_index = 0;
                    }

                    // Check if the node contains the "end_points" key.
                    if let Some(endpoints) = node.as_object_mut().unwrap().get_mut(&"end_points".to_string()) {

                        // Retrieve the array of endpoints.
                        let endpoints_array: &mut Vec<Value> = endpoints.as_array_mut().unwrap();
                        
                        // Iterate over each endpoint.
                        for endpoint in endpoints_array {

                            // Check if the endpoint contains the "lower_connections" key.
                            if let Some(lower_conns_ref) = endpoint.clone().as_object().unwrap().get(&"lower_connections".to_string()) {

                                // Retrieve the array of lower connections.
                                let lower_conns_array = lower_conns_ref.as_array().unwrap();
                                
                                // Add individual fields for each lower connection.
                                for lower_conn_index in 0..lower_conns_array.len() {
                                    endpoint.as_object_mut().unwrap().insert(
                                        format!("lower_connection_{}", lower_conn_index), 
                                        lower_conns_array[lower_conn_index].as_object().unwrap().get(&"connection-uuid".to_string()).unwrap().clone()
                                    );
                                }
                            }

                            // Remove the "lower_connections" field from the endpoint.
                            endpoint.as_object_mut().unwrap().remove(&"lower_connections".to_string());
                        }

                        // Sort endpoints by their "id" field.
                        endpoints.as_array_mut().unwrap().sort_by(|a, b| a.as_object().unwrap().get(&"id".to_string()).unwrap().as_i64()
                            .cmp(&b.as_object().unwrap().get(&"id".to_string()).unwrap().as_i64()));

                    } else {
                        return Err(error::ErrorNotFound("Cannot find end_points on nodes"));
                    }
                }

                // Sort nodes by their "node_id" field.
                nodes.as_array_mut().unwrap().sort_by(|a, b| a.as_object().unwrap().get(&"node_id".to_string()).unwrap().as_i64()
                .cmp(&b.as_object().unwrap().get(&"node_id".to_string()).unwrap().as_i64()));

            } else {
                return Err(error::ErrorNotFound("Cannot find nodes on connectivity_services"));
            }
        }

    } else {
        return Err(error::ErrorNotFound("Cannot find connectivity_services on schema"));
    }

    Ok(())
}
