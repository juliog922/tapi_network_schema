use serde_json::Value;

use super::endpoint::BaseEndpoint;
use crate::utils::find_name;

#[derive(Debug, Clone)]
pub struct Connection {
    pub connection_uuid: String,
    pub name: String,
    pub lower_connections: Vec<LowerConnection>,
    pub connection_end_points: Vec<CConnectionEndPoint>,
}

#[derive(Debug, Clone)]
pub struct LowerConnection {
    pub connection_uuid: String,
}

#[derive(Debug, Clone)]
pub struct CConnectionEndPoint {
    pub node_edge_point_uuid: String,
    pub connection_end_point_uuid: String,
    pub node_uuid: String,
}

enum ConnectionRole {
    UpperWithLowers,
    UpperWithoutLowers,
    Lower,
}

impl Connection {
    pub fn provide_connection(
        &self,
        base_endpoint: &mut BaseEndpoint,
        connection_vector: &Vec<Self>,
    ) -> Vec<BaseEndpoint> {
        let mut base_endpoint_vector: Vec<BaseEndpoint> = Vec::new();

        if self.connection_end_points.iter().any(|cep| {
            cep.node_edge_point_uuid == base_endpoint.node_edge_point_uuid
        }) {
            // Actualiza base_endpoint con datos relevantes
            if let Some(cep) = self.connection_end_points.iter().find(|cep| {
                cep.node_edge_point_uuid == base_endpoint.node_edge_point_uuid
            }) {
                base_endpoint.connection_end_point_uuid = Some(cep.connection_end_point_uuid.clone());
            }

            // Determina el rol de la conexión
            match self.determine_connection_role(connection_vector, base_endpoint) {
                ConnectionRole::UpperWithLowers => {
                    base_endpoint.connection_uuid = Some(self.connection_uuid.clone());

                    // Agrega otros extremos
                    base_endpoint_vector.extend(self.generate_base_endpoints(
                        base_endpoint,
                        &base_endpoint.node_edge_point_uuid,
                        None,
                        None,
                        1,
                    ));

                    // Procesa conexiones inferiores
                    for lower in &self.lower_connections {
                        if let Some(conn) = connection_vector.iter().find(|c| {
                            c.is_right_lower_connection(&lower.connection_uuid, base_endpoint)
                        }) {
                            base_endpoint.lower_connection = Some(lower.connection_uuid.clone());
                            base_endpoint_vector.extend(conn.generate_base_endpoints(
                                base_endpoint,
                                &base_endpoint.node_edge_point_uuid,
                                Some(conn.connection_uuid.clone()),
                                None,
                                2,
                            ));
                        }
                    }
                }
                ConnectionRole::UpperWithoutLowers => {
                    base_endpoint.connection_uuid = Some(self.connection_uuid.clone());
                    base_endpoint_vector.extend(self.generate_base_endpoints(
                        base_endpoint,
                        &base_endpoint.node_edge_point_uuid,
                        None,
                        None,
                        1,
                    ));
                }
                ConnectionRole::Lower => {
                    base_endpoint.connection_uuid = Some(self.connection_uuid.clone());
                    base_endpoint_vector.extend(self.generate_base_endpoints(
                        base_endpoint,
                        &base_endpoint.node_edge_point_uuid,
                        None,
                        Some(self.connection_uuid.clone()),
                        -2,
                    ));
                }
            }
        }

        base_endpoint_vector
    }

    fn determine_connection_role(
        &self,
        connection_vector: &Vec<Connection>,
        base_endpoint: &BaseEndpoint,
    ) -> ConnectionRole {
        if !self.lower_connections.is_empty() {
            ConnectionRole::UpperWithLowers
        } else if connection_vector.iter().any(|conn| {
            conn.lower_connections.iter().any(|lc| lc.connection_uuid == self.connection_uuid)
        }) {
            ConnectionRole::Lower
        } else {
            ConnectionRole::UpperWithoutLowers
        }
    }

    fn generate_base_endpoints(
        &self,
        base_endpoint: &BaseEndpoint,
        excluded_uuid: &str,
        new_connection_uuid: Option<String>,
        new_lower_connection: Option<String>,
        id_offset: i32,
    ) -> Vec<BaseEndpoint> {
        self.connection_end_points
            .iter()
            .filter(|cep| cep.node_edge_point_uuid != excluded_uuid)
            .map(|cep| BaseEndpoint {
                node_edge_point_uuid: cep.node_edge_point_uuid.clone(),
                node_uuid: cep.node_uuid.clone(),
                connection_end_point_uuid: Some(cep.connection_end_point_uuid.clone()),
                connection_uuid: new_connection_uuid.clone(),
                lower_connection: new_lower_connection.clone(),
                id: base_endpoint.id.map(|id| id + id_offset),
                ..Default::default()
            })
            .collect()
    }

    pub fn is_right_lower_connection(&self, lower_connection_uuid: &str, base_endpoint: &BaseEndpoint) -> bool {
        self.connection_uuid == lower_connection_uuid
            && self.connection_end_points.iter().any(|cep| cep.node_edge_point_uuid == base_endpoint.node_edge_point_uuid)
    }
}

pub fn connection_vector_build(connections_json: &Vec<Value>) -> Vec<Connection> {

    let mut connection_vector: Vec<Connection> = Vec::new();

    for connection_item in connections_json {

        let mut connection_end_point_vector: Vec<CConnectionEndPoint> = Vec::new();

        if let Some(connection_end_point_section) = connection_item.get("connection-end-point").and_then(Value::as_array) {
            for connection_end_point_item in connection_end_point_section {

                connection_end_point_vector.push(
                    CConnectionEndPoint {
                        node_edge_point_uuid: connection_end_point_item.get("node-edge-point-uuid").unwrap_or(&Value::default()).to_string(),
                        connection_end_point_uuid: connection_end_point_item.get("connection-end-point-uuid").unwrap_or(&Value::default()).to_string(),
                        node_uuid: connection_end_point_item.get("node-uuid").unwrap_or(&Value::default()).to_string(),
                    }
                );

            }
        }

        let mut lower_connection_vector: Vec<LowerConnection> = Vec::new();

        if let Some(lower_connection_section) = connection_item.get("lower-connection").and_then(Value::as_array) {
            for lower_connection_item in lower_connection_section {

                lower_connection_vector.push(
                    LowerConnection {
                        connection_uuid: lower_connection_item.get("connection-uuid").unwrap_or(&Value::default()).to_string(),
                    }
                );

            }
        }

        connection_vector.push(
            Connection {
                connection_uuid: connection_item.get("uuid").unwrap_or(&Value::default()).to_string(),
                name: find_name(connection_item, "CONNECTION_NAME".to_string()),
                lower_connections: lower_connection_vector,
                connection_end_points: connection_end_point_vector,
            }
        );

    }

    connection_vector
}