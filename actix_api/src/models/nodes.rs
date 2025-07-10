use serde::{Deserialize, Serialize};

use super::endpoint::BaseEndpoint;

/// Represents a node in the network with associated edge points and a unique identifier.
#[derive(Debug, Clone)]
pub struct Node {
    pub node_uuid: String,
    pub name: String,
    pub owned_node_edge_points: Vec<OwnedNodeEdgePoint>,
}

/// Represents an edge point owned by a node.
#[derive(Debug, Clone)]
pub struct OwnedNodeEdgePoint {
    pub node_edge_point_uuid: String,
    pub inventory_id: String,
    pub connection_end_points: Vec<NodeConnectionEndPoint>,
    pub mc_pool: Option<McPool>,
}

/// Represents an mc pool of an owned-node-edge-point.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct McPool {
    pub occupied_spectrum: Option<Vec<FrecuencyPair>>,
    pub available_spectrum:Option< Vec<FrecuencyPair>>,
}

/// Represents an edge point owned by a node.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FrecuencyPair {
    pub upper_frequency: i64,
    pub lower_frequency: i64,
}

/// Represents a connection endpoint within a node.
#[derive(Debug, Clone)]
pub struct NodeConnectionEndPoint {
    pub connection_end_point_uuid: String,
    pub layer_protocol_qualifier: String,
    pub client_node_edge_points: Vec<ClientNodeEdgePoint>,
}

/// Represents a client node edge point, which references another node.
#[derive(Debug, Clone)]
pub struct ClientNodeEdgePoint {
    pub node_edge_point_uuid: String,
    pub node_uuid: String,
}

impl Node {
    /// Resolves inventory and related information for a base endpoint and generates new base endpoints.
    ///
    /// # Arguments
    /// - `base_endpoint`: A mutable reference to the `BaseEndpoint` being processed.
    ///
    /// # Returns
    /// A vector of new `BaseEndpoint` objects based on the node's relationships and inventory.
    pub fn provide_inventory(&self, base_endpoint: &mut BaseEndpoint) -> Vec<BaseEndpoint> {
        let mut base_endpoint_vector = Vec::new();

        if self.node_uuid == base_endpoint.node_uuid {
            for owned_node_edge_point in &self.owned_node_edge_points {
                if owned_node_edge_point.node_edge_point_uuid == base_endpoint.node_edge_point_uuid
                {
                    base_endpoint.inventory_id = Some(owned_node_edge_point.inventory_id.clone());
                    base_endpoint.mc_pool = owned_node_edge_point.mc_pool.clone();
                        
                    for connection_end_point in owned_node_edge_point.connection_end_points.iter() {
                        if base_endpoint.connection_end_point_uuid.is_none()
                            || base_endpoint.connection_end_point_uuid.clone().unwrap()
                                == connection_end_point.connection_end_point_uuid
                        {
                            base_endpoint.layer_protocol_qualifier =
                                Some(connection_end_point.layer_protocol_qualifier.clone());
                            base_endpoint.connection_end_point_uuid =
                                Some(connection_end_point.connection_end_point_uuid.clone());

                            if connection_end_point.client_node_edge_points.len() > 1 {
                                println!("There is more than 2 Clients for the same Endpoint for node-edge-point-uuid: {}", base_endpoint.node_edge_point_uuid);
                            }

                            if !connection_end_point.client_node_edge_points.is_empty() {
                                base_endpoint.client_node_edge_point_uuid = Some(
                                    connection_end_point.client_node_edge_points[0]
                                        .node_edge_point_uuid
                                        .clone(),
                                );

                                let possible_id = base_endpoint.id.map(|id| id + 1);

                                base_endpoint_vector.push(BaseEndpoint {
                                    node_edge_point_uuid: connection_end_point
                                        .client_node_edge_points[0]
                                        .node_edge_point_uuid
                                        .clone(),
                                    node_uuid: connection_end_point.client_node_edge_points[0]
                                        .node_uuid
                                        .clone(),
                                    mc_pool: None,
                                    connection_end_point_uuid: None,
                                    service_interface_point_uuid: None,
                                    connection_uuid: None,
                                    client_node_edge_point_uuid: None,
                                    lower_connection: None,
                                    link_uuid: None,
                                    layer_protocol_qualifier: None,
                                    inventory_id: None,
                                    id: possible_id,
                                });
                            }
                        }
                    }
                }
            }
        }
        // Add parent edge points for the base endpoint.
        for owned_node_edge_point in &self.owned_node_edge_points {
            if !owned_node_edge_point.connection_end_points.is_empty()
                && !owned_node_edge_point.connection_end_points[0]
                    .client_node_edge_points
                    .is_empty()
                && owned_node_edge_point.connection_end_points[0].client_node_edge_points[0]
                    .node_edge_point_uuid
                    == base_endpoint.node_edge_point_uuid
            {
                let possible_id = base_endpoint.id.map(|id| id + 1);
                base_endpoint_vector.push(BaseEndpoint {
                    node_edge_point_uuid: owned_node_edge_point.node_edge_point_uuid.clone(),
                    mc_pool: None,
                    node_uuid: self.node_uuid.clone(),
                    connection_end_point_uuid: None,
                    service_interface_point_uuid: None,
                    connection_uuid: None,
                    client_node_edge_point_uuid: None,
                    lower_connection: None,
                    link_uuid: None,
                    layer_protocol_qualifier: None,
                    inventory_id: None,
                    id: possible_id,
                });
            }
        }

        base_endpoint_vector
    }
}
