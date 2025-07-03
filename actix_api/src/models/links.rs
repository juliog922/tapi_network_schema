use super::endpoint::BaseEndpoint;

/// Represents a link in the network, connecting multiple node edge points.
#[derive(Debug, Clone)]
pub struct Link {
    pub link_uuid: String,
    pub node_edge_points: Vec<NodeEdgePoint>,
}

/// Represents a node edge point, which belongs to a specific node.
#[derive(Debug, Clone)]
pub struct NodeEdgePoint {
    pub node_edge_point_uuid: String,
    pub node_uuid: String,
}

impl Link {
    /// Resolves the relationship between a link and a base endpoint, and generates associated `BaseEndpoint` objects.
    ///
    /// # Arguments
    /// - `base_endpoint`: A mutable reference to the `BaseEndpoint` being processed.
    ///
    /// # Returns
    /// A vector of new `BaseEndpoint` objects derived from the link's other node edge points.
    pub fn provide_link(&self, base_endpoint: &mut BaseEndpoint) -> Vec<BaseEndpoint> {
        let mut base_endpoint_vector = Vec::new();

        // Check if the base endpoint matches any node edge point in the link.
        if self.node_edge_points.iter().any(|node_edge_point| {
            node_edge_point.node_edge_point_uuid == base_endpoint.node_edge_point_uuid
        }) {
            base_endpoint.link_uuid = Some(self.link_uuid.clone());

            let possible_id = base_endpoint.id.map(|id| id + 1);

            // Create new `BaseEndpoint` objects for node edge points not matching the base endpoint.
            base_endpoint_vector = self
                .node_edge_points
                .iter()
                .filter(|node_edge_point| {
                    node_edge_point.node_edge_point_uuid != base_endpoint.node_edge_point_uuid
                })
                .map(|node_edge_point| BaseEndpoint {
                    node_edge_point_uuid: node_edge_point.node_edge_point_uuid.clone(),
                    node_uuid: node_edge_point.node_uuid.clone(),
                    mc_pool: None,
                    connection_end_point_uuid: None,
                    service_interface_point_uuid: None,
                    connection_uuid: None,
                    client_node_edge_point_uuid: None,
                    lower_connection: None,
                    link_uuid: Some(self.link_uuid.clone()),
                    inventory_id: None,
                    layer_protocol_qualifier: None,
                    id: possible_id,
                })
                .collect();
        }

        base_endpoint_vector
    }
}
