use serde_json::Value;

use crate::{
    models::connections::{CConnectionEndPoint, Connection, LowerConnection},
    utils::find_name,
};

/// Constructs a vector of `Connection` objects from a vector of JSON values.
///
/// # Arguments
///
/// * `connections_json` - A reference to a vector of JSON values, where each value
///   represents a connection's data.
///
/// # Returns
///
/// A vector of `Connection` objects built from the provided JSON data.
pub fn connection_vector_build(connections_json: &Vec<Value>) -> Vec<Connection> {
    // Initialize the vector to store the resulting `Connection` objects.
    let mut connection_vector: Vec<Connection> = Vec::new();

    for connection_item in connections_json {
        // Vector to hold `CConnectionEndPoint` objects for the current connection.
        let mut connection_end_point_vector: Vec<CConnectionEndPoint> = Vec::new();

        // Check if the "connection-end-point" key exists and is an array.
        if let Some(connection_end_point_section) = connection_item
            .get("connection-end-point")
            .and_then(Value::as_array)
        {
            for connection_end_point_item in connection_end_point_section {
                // Push a new `CConnectionEndPoint` object into the vector.
                // Use `unwrap_or` to handle missing keys with a default value.
                connection_end_point_vector.push(CConnectionEndPoint {
                    node_edge_point_uuid: connection_end_point_item
                        .get("node-edge-point-uuid")
                        .unwrap_or(&Value::default())
                        .to_string(),
                    connection_end_point_uuid: connection_end_point_item
                        .get("connection-end-point-uuid")
                        .unwrap_or(&Value::default())
                        .to_string(),
                    node_uuid: connection_end_point_item
                        .get("node-uuid")
                        .unwrap_or(&Value::default())
                        .to_string(),
                });
            }
        }

        // Vector to hold `LowerConnection` objects for the current connection.
        let mut lower_connection_vector: Vec<LowerConnection> = Vec::new();

        // Check if the "lower-connection" key exists and is an array.
        if let Some(lower_connection_section) = connection_item
            .get("lower-connection")
            .and_then(Value::as_array)
        {
            for lower_connection_item in lower_connection_section {
                // Push a new `LowerConnection` object into the vector.
                lower_connection_vector.push(LowerConnection {
                    connection_uuid: lower_connection_item
                        .get("connection-uuid")
                        .unwrap_or(&Value::default())
                        .to_string(),
                });
            }
        }

        // Add the constructed `Connection` object to the result vector.
        connection_vector.push(Connection {
            connection_uuid: connection_item
                .get("uuid")
                .unwrap_or(&Value::default())
                .to_string(),
            // Use the utility function `find_name` to find the connection's name.
            name: find_name(connection_item, "CONNECTION_NAME".to_string()),
            lower_connections: lower_connection_vector,
            connection_end_points: connection_end_point_vector,
        });
    }

    connection_vector
}
