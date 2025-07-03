use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    connections::{Connection, LowerConnection},
    endpoint::BaseEndpoint,
};
use crate::utils::find_name;

/// Represents a connectivity service, including its endpoints and associated connections.
#[derive(Debug, Clone)]
pub struct Service {
    pub service_uuid: String,
    pub name: String,
    pub end_points: Vec<EndPoint>,
    pub connections: Vec<ServiceConnection>,
    pub lower_connections: Vec<LowerConnection>,
}

/// Represents an endpoint within a service.
#[derive(Debug, Clone)]
pub struct EndPoint {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub location: String,
    pub connection_end_points: Vec<ServiceConnectionEndPoint>,
    pub service_interface_point_uuid: String,
}

/// Represents a direct connection in a service.
#[derive(Debug, Clone)]
pub struct ServiceConnection {
    pub connection_uuid: String,
}

/// Represents a connection endpoint associated with a node.
#[derive(Debug, Clone)]
pub struct ServiceConnectionEndPoint {
    pub node_edge_point_uuid: String,
    pub connection_end_point_uuid: String,
    pub node_uuid: String,
}

/// Represents a simplified version of a service with minimal data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleService {
    pub uuid: String,
    pub name: String,
}

impl Service {
    /// Constructs the initial vector of `BaseEndpoint` objects from the service's endpoints.
    ///
    /// # Returns
    /// A vector of `BaseEndpoint` objects with basic initialization.
    pub fn first_base_endpoint_vector(&self) -> Vec<BaseEndpoint> {
        let mut base_endpoint_vector = Vec::new();
        let id: i32 = 1;

        self.end_points.iter().for_each(|end_point| {
            end_point
                .connection_end_points
                .iter()
                .for_each(|connection_end_point| {
                    base_endpoint_vector.push(BaseEndpoint {
                        node_edge_point_uuid: connection_end_point.node_edge_point_uuid.clone(),
                        node_uuid: connection_end_point.node_uuid.clone(),
                        connection_end_point_uuid: Some(
                            connection_end_point.connection_end_point_uuid.clone(),
                        ),
                        service_interface_point_uuid: Some(
                            end_point.service_interface_point_uuid.clone(),
                        ),
                        mc_pool: None,
                        connection_uuid: None,
                        client_node_edge_point_uuid: None,
                        lower_connection: None,
                        link_uuid: None,
                        layer_protocol_qualifier: None,
                        inventory_id: None,
                        id: Some(id),
                    });
                    //id += 1;
                });
        });
        base_endpoint_vector
    }

    /// Constructs a `Service` object from JSON and a list of available connections.
    ///
    /// # Arguments
    /// - `connectivity_service_json`: The JSON object representing the service.
    /// - `connection_vector`: A vector of available `Connection` objects.
    ///
    /// # Returns
    /// A `Service` object with initialized properties.
    pub fn connectivity_service_build(
        connectivity_service_json: &Value,
        connection_vector: &Vec<Connection>,
    ) -> Self {
        let mut end_point_vector: Vec<EndPoint> = Vec::new();

        if let Some(end_point_section) = connectivity_service_json
            .get("end-point")
            .and_then(Value::as_array)
        {
            for end_point_item in end_point_section {
                let mut service_connection_end_point_vector: Vec<ServiceConnectionEndPoint> =
                    Vec::new();

                // Parse "connection-end-point" for each endpoint.
                if let Some(connection_end_point_section) = end_point_item
                    .get("connection-end-point")
                    .and_then(Value::as_array)
                {
                    for connection_end_point_item in connection_end_point_section {
                        service_connection_end_point_vector.push(ServiceConnectionEndPoint {
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

                end_point_vector.push(EndPoint {
                    name: find_name(end_point_item, "CSEP_NAME".to_string()),
                    location: find_name(end_point_item, "location".to_string()),
                    connection_end_points: service_connection_end_point_vector,
                    service_interface_point_uuid: end_point_item
                        .get("service-interface-point")
                        .unwrap_or(&Value::default())
                        .get("service-interface-point-uuid")
                        .unwrap_or(&Value::default())
                        .to_string(),
                });
            }
        }

        let mut service_connection_vector: Vec<ServiceConnection> = Vec::new();
        let mut service_lower_connection_vector: Vec<LowerConnection> = Vec::new();

        // Parse the "connection" section to build service connections and lower connections.
        if let Some(connection_section) = connectivity_service_json
            .get("connection")
            .and_then(Value::as_array)
        {
            for connection in connection_section {
                let connection_uuid = connection
                    .get("connection-uuid")
                    .unwrap_or(&Value::default())
                    .to_string();
                service_connection_vector.push(ServiceConnection {
                    connection_uuid: connection_uuid.clone(),
                });

                // Find lower connections for the current connection.
                'lower_loop: for connection_struct in connection_vector {
                    if connection_struct.connection_uuid == connection_uuid {
                        service_lower_connection_vector
                            .extend(connection_struct.lower_connections.clone());
                        break 'lower_loop; // Exit loop early as match is found.
                    }
                }
            }
        }

        Self {
            service_uuid: connectivity_service_json
                .get("uuid")
                .unwrap_or(&Value::default())
                .to_string(),
            name: find_name(connectivity_service_json, "SERVICE_NAME".to_string()),
            end_points: end_point_vector,
            connections: service_connection_vector,
            lower_connections: service_lower_connection_vector,
        }
    }
}
