use serde::{Deserialize, Serialize};

use crate::models::nodes::{FrecuencyPair, McPool};

use super::{connections::Connection, links::Link, nodes::Node};

/// Represents a detailed endpoint with various optional fields and associated metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub node_edge_point_uuid: String,
    pub layer_protocol_qualifier: String,
    pub node_uuid: String,
    pub inventory_id: String,
    pub connection_end_point_uuid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_interface_point_uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_node_edge_point_uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_connection: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mc_pool: Option<McPool>,

    /// Unique identifier for the endpoint.
    pub id: i32,
}

/// Represents a basic endpoint with optional fields, often used as a starting point.
#[derive(Debug, Clone, Eq, Serialize, Deserialize, Default)]
pub struct BaseEndpoint {
    pub node_edge_point_uuid: String,
    pub node_uuid: String,
    pub connection_end_point_uuid: Option<String>,
    pub service_interface_point_uuid: Option<String>,
    pub connection_uuid: Option<String>,
    pub client_node_edge_point_uuid: Option<String>,
    pub lower_connection: Option<String>,
    pub link_uuid: Option<String>,
    pub layer_protocol_qualifier: Option<String>,
    pub mc_pool: Option<McPool>,

    /// INVENTORY ID (card id)
    pub inventory_id: Option<String>,

    /// Unique identifier for the endpoint.
    pub id: Option<i32>,
}

impl PartialEq for BaseEndpoint {
    fn eq(&self, other: &Self) -> bool {
        self.node_edge_point_uuid == other.node_edge_point_uuid
    }
}

impl BaseEndpoint {
    /// Builds an `Endpoint` and associated `BaseEndpoint` objects by resolving connections, links, and inventories.
    ///
    /// # Arguments
    /// - `link_vector`: A vector of links to check for associations.
    /// - `node_vector`: A vector of nodes to resolve inventories.
    /// - `connection_vector`: A vector of connections to determine relationships.
    ///
    /// # Returns
    /// A tuple containing the constructed `Endpoint` and a deduplicated vector of `BaseEndpoint` objects.
    pub fn build(
        mut self,
        link_vector: &Vec<Link>,
        node_vector: &Vec<Node>,
        connection_vector: &Vec<Connection>,
    ) -> (Endpoint, Vec<Self>) {
        let mut base_endpoint_vector: Vec<Self> = Vec::new();

        if self.connection_uuid.is_none() {
            // Loop through connections to find matching ones and extend the base endpoint vector.
            'connection_loop: for connection in connection_vector {
                let connection_base_endpoint_vector =
                    connection.provide_connection(&mut self, connection_vector);
                if !connection_base_endpoint_vector.is_empty() {
                    base_endpoint_vector.extend(connection_base_endpoint_vector);
                    if self.connection_uuid.is_some() {
                        break 'connection_loop; // Stop early if connection UUID is found.
                    }
                }
            }
        }

        if self.link_uuid.is_none() {
            // Loop through links to find matches and extend the base endpoint vector.
            'link_loop: for link in link_vector {
                let link_base_endpoint_vector = link.provide_link(&mut self);
                if !link_base_endpoint_vector.is_empty() {
                    base_endpoint_vector.extend(link_base_endpoint_vector);
                    if self.link_uuid.is_some() {
                        break 'link_loop; // Stop early if link UUID is found.
                    }
                }
            }
        }

        if self.inventory_id.is_none() {
            // Check nodes to resolve inventory IDs.
            for node in node_vector {
                let node_base_endpoint_vector = node.provide_inventory(&mut self);
                if !node_base_endpoint_vector.is_empty() {
                    base_endpoint_vector.extend(node_base_endpoint_vector);
                }
            }
        }

        // Construct the final `Endpoint` object.
        let endpoint = Endpoint {
            node_edge_point_uuid: self.node_edge_point_uuid,
            layer_protocol_qualifier: self.layer_protocol_qualifier.unwrap_or_default(),
            node_uuid: self.node_uuid,
            inventory_id: self.inventory_id.unwrap_or_default(),
            connection_end_point_uuid: self.connection_end_point_uuid.unwrap_or_default(),
            service_interface_point_uuid: self.service_interface_point_uuid,
            connection_uuid: self.connection_uuid,
            client_node_edge_point_uuid: self.client_node_edge_point_uuid,
            lower_connection: self.lower_connection,
            link_uuid: self.link_uuid,
            mc_pool: self.mc_pool,
            id: self.id.unwrap_or_default(),
        };

        // Deduplicate the resulting `BaseEndpoint` vector.
        let mut seen_node_edge_point_uuids = Vec::new();
        let mut unique_base_endpoints = Vec::new();

        for base_endpoint in base_endpoint_vector {
            if !seen_node_edge_point_uuids.contains(&base_endpoint.node_edge_point_uuid) {
                seen_node_edge_point_uuids.push(base_endpoint.node_edge_point_uuid.clone());
                unique_base_endpoints.push(base_endpoint);
            }
        }

        // Devolver el Endpoint y el vector deduplicado de BaseEndpoint
        (endpoint, unique_base_endpoints)
    }
}
