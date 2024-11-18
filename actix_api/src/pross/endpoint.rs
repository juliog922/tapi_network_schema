use serde::{Deserialize, Serialize};

use super::{connections::Connection, links::Link, nodes::Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    /// UUID of the node edge point.
    pub node_edge_point_uuid: String,

    /// Layer protocol qualifier associated with this endpoint.
    pub layer_protocol_qualifier: String,

    ///Node UUID
    pub node_uuid: String,

    /// INVENTORY ID (card id)
    pub inventory_id: String,

    /// UUID of the connection endpoint.
    pub connection_end_point_uuid: String,

    /// Optional UUID of the service interface point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_interface_point_uuid: Option<String>,

    /// Optional UUID of the connection associated with this endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_uuid: Option<String>,

    /// Optional UUID of the client node edge point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_node_edge_point_uuid: Option<String>,

    /// Optional field representing lower connections related to this endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_connection: Option<String>,

    /// Optional UUID of the link associated with this endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_uuid: Option<String>,

    /// Unique identifier for the endpoint.
    pub id: i32,
}

#[derive(Debug, Clone, Eq, Serialize, Deserialize, Default)]
pub struct BaseEndpoint {
    /// UUID of the node edge point.
    pub node_edge_point_uuid: String,

    ///Node UUID
    pub node_uuid: String,

    /// UUID of the connection endpoint.
    pub connection_end_point_uuid: Option<String>,

    /// Optional UUID of the service interface point.
    pub service_interface_point_uuid: Option<String>,

    /// Optional UUID of the connection associated with this endpoint.
    pub connection_uuid: Option<String>,

    /// Optional UUID of the client node edge point.
    pub client_node_edge_point_uuid: Option<String>,

    /// Optional field representing lower connections related to this endpoint.
    pub lower_connection: Option<String>,

    /// Optional UUID of the link associated with this endpoint.
    pub link_uuid: Option<String>,

    /// Layer protocol qualifier associated with this endpoint.
    pub layer_protocol_qualifier: Option<String>,

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
    pub fn build(mut self, link_vector: &Vec<Link>, node_vector: &Vec<Node>, connection_vector: &Vec<Connection>) -> (Endpoint, Vec<Self>) {
        let mut base_endpoint_vector: Vec<Self> = Vec::new();

        if self.link_uuid.is_none() {
            'link_loop: for link in link_vector {
                let link_base_endpoint_vector = link.provide_link(&mut self);
                if !link_base_endpoint_vector.is_empty() {
                    base_endpoint_vector.extend(link_base_endpoint_vector);
                    if !self.link_uuid.is_none() {
                        break 'link_loop;
                    }
                }
            }
        }

        if self.connection_uuid.is_none() {
            'connection_loop: for connection in connection_vector {
                let connection_base_endpoint_vector = connection.provide_connection(&mut self, connection_vector);
                if !connection_base_endpoint_vector.is_empty() {
                    base_endpoint_vector.extend(connection_base_endpoint_vector);
                    if !self.connection_uuid.is_none() {
                        break 'connection_loop;
                    }
                }
            }
        }

        // Cant 
        if self.inventory_id.is_none() {
            for node in node_vector {
                let node_base_endpoint_vector = node.provide_inventory(&mut self);
                if !node_base_endpoint_vector.is_empty() {
                    base_endpoint_vector.extend(node_base_endpoint_vector);
                    
                }
            }
        }

        // Construcción del Endpoint
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
            id: self.id.unwrap_or_default(),
        };

        // Realizar deduplicación manual
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
