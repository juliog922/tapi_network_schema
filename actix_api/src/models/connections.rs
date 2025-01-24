use super::endpoint::BaseEndpoint;

/// Represents a connection in the network.
#[derive(Debug, Clone)]
pub struct Connection {
    pub connection_uuid: String,
    #[allow(dead_code)]
    pub name: String,
    pub lower_connections: Vec<LowerConnection>,
    pub connection_end_points: Vec<CConnectionEndPoint>,
}

/// Represents a lower connection that is part of a higher-level connection.
#[derive(Debug, Clone)]
pub struct LowerConnection {
    pub connection_uuid: String,
}

/// Represents a connection endpoint.
#[derive(Debug, Clone)]
pub struct CConnectionEndPoint {
    pub node_edge_point_uuid: String,
    pub connection_end_point_uuid: String,
    pub node_uuid: String,
}

/// Enum representing the role of a connection relative to others.
enum ConnectionRole {
    UpperWithLowers,
    UpperWithoutLowers,
    Lower,
    UnknownUpper,
}

impl Connection {
    /// Processes the connection and provides a vector of `BaseEndpoint` objects.
    ///
    /// # Arguments
    /// - `base_endpoint`: A mutable reference to the `BaseEndpoint` being processed.
    /// - `connection_vector`: A vector of all available connections.
    ///
    /// # Returns
    /// A vector of generated `BaseEndpoint` objects based on the connection's role and associated endpoints.
    pub fn provide_connection(
        &self,
        base_endpoint: &mut BaseEndpoint,
        connection_vector: &[Self],
    ) -> Vec<BaseEndpoint> {
        let connection_vector: Vec<Connection> = connection_vector
            .to_owned()
            .clone()
            .into_iter()
            .filter(|connection| connection.connection_uuid != self.connection_uuid)
            .collect();
        let mut base_endpoint_vector: Vec<BaseEndpoint> = Vec::new();

        if self
            .connection_end_points
            .iter()
            .any(|cep| cep.node_edge_point_uuid == base_endpoint.node_edge_point_uuid)
        {
            // Match the base endpoint with a connection endpoint and update it.
            if let Some(cep) = self
                .connection_end_points
                .iter()
                .find(|cep| cep.node_edge_point_uuid == base_endpoint.node_edge_point_uuid)
            {
                base_endpoint.connection_end_point_uuid =
                    Some(cep.connection_end_point_uuid.clone());
            }

            // Determine the connection's role and process accordingly.
            match self.determine_connection_role(&connection_vector, base_endpoint) {
                ConnectionRole::UpperWithLowers => {
                    base_endpoint.connection_uuid = Some(self.connection_uuid.clone());

                    // Generate new base endpoints for the connection's endpoints.
                    base_endpoint_vector.extend(self.generate_base_endpoints(
                        base_endpoint,
                        &base_endpoint.node_edge_point_uuid,
                        1,
                    ));

                    // Process lower connections recursively.
                    for lower in &self.lower_connections {
                        if let Some(conn) = connection_vector.iter().find(|c| {
                            c.is_right_lower_connection(&lower.connection_uuid, base_endpoint)
                        }) {
                            base_endpoint.lower_connection = Some(lower.connection_uuid.clone());
                            base_endpoint_vector.extend(conn.generate_base_endpoints(
                                base_endpoint,
                                &base_endpoint.node_edge_point_uuid,
                                1,
                            ));
                        }
                    }
                }
                ConnectionRole::UpperWithoutLowers => {
                    base_endpoint.connection_uuid = Some(self.connection_uuid.clone());
                    base_endpoint_vector.extend(self.generate_base_endpoints(
                        base_endpoint,
                        &base_endpoint.node_edge_point_uuid,
                        1,
                    ));
                }
                ConnectionRole::Lower => {
                    base_endpoint.connection_uuid = Some(self.connection_uuid.clone());
                    base_endpoint_vector.extend(self.generate_base_endpoints(
                        base_endpoint,
                        &base_endpoint.node_edge_point_uuid,
                        1,
                    ));
                }
                ConnectionRole::UnknownUpper => {}
            }
        }

        base_endpoint_vector
    }

    /// Determines the role of the connection relative to other connections in the network.
    ///
    /// # Arguments
    /// - `connection_vector`: A reference to a vector of all available connections.
    /// - `base_endpoint`: A reference to the `BaseEndpoint` being evaluated.
    ///
    /// # Returns
    /// A `ConnectionRole` enum indicating the role of the connection:
    /// - `UpperWithLowers`: The connection has dependent lower connections.
    /// - `Lower`: The connection is a lower connection in a hierarchy.
    /// - `UpperWithoutLowers`: The connection has no dependent lower connections.
    /// - `UnknownUpper`: The connection's role could not be determined.
    fn determine_connection_role(
        &self,
        connection_vector: &[Connection],
        base_endpoint: &BaseEndpoint,
    ) -> ConnectionRole {
        if !self.lower_connections.is_empty() {
            ConnectionRole::UpperWithLowers
        } else if connection_vector.iter().any(|conn| {
            conn.lower_connections
                .iter()
                .any(|lc| lc.connection_uuid == self.connection_uuid)
                && conn
                    .connection_end_points
                    .iter()
                    .all(|cep| cep.node_edge_point_uuid != base_endpoint.node_edge_point_uuid)
        }) {
            ConnectionRole::Lower
        } else if !connection_vector.iter().any(|conn| {
            conn.lower_connections
                .iter()
                .any(|lc| lc.connection_uuid == self.connection_uuid)
        }) {
            ConnectionRole::UpperWithoutLowers
        } else {
            ConnectionRole::UnknownUpper
        }
    }

    /// Generates a vector of new `BaseEndpoint` objects based on connection endpoints.
    ///
    /// # Arguments
    /// - `excluded_uuid`: Node edge point UUID to exclude from the results.
    /// - `id_offset`: Offset to apply to the IDs of the generated endpoints.
    fn generate_base_endpoints(
        &self,
        base_endpoint: &BaseEndpoint,
        excluded_uuid: &str,
        id_offset: i32,
    ) -> Vec<BaseEndpoint> {
        self.connection_end_points
            .iter()
            .filter(|cep| cep.node_edge_point_uuid != excluded_uuid)
            .map(|cep| BaseEndpoint {
                node_edge_point_uuid: cep.node_edge_point_uuid.clone(),
                node_uuid: cep.node_uuid.clone(),
                connection_end_point_uuid: Some(cep.connection_end_point_uuid.clone()),
                id: base_endpoint.id.map(|id| id + id_offset),
                ..Default::default()
            })
            .collect()
    }

    /// Determines if the current connection is the correct lower connection.
    ///
    /// # Arguments
    /// - `lower_connection_uuid`: The UUID of the lower connection to check.
    /// - `base_endpoint`: The base endpoint to compare against.
    ///
    /// # Returns
    /// `true` if the current connection matches the lower connection and the base endpoint; otherwise, `false`.
    pub fn is_right_lower_connection(
        &self,
        lower_connection_uuid: &str,
        base_endpoint: &BaseEndpoint,
    ) -> bool {
        self.connection_uuid == lower_connection_uuid
            && self
                .connection_end_points
                .iter()
                .any(|cep| cep.node_edge_point_uuid == base_endpoint.node_edge_point_uuid)
    }
}
