use serde::{Deserialize, Serialize};

use super::endpoint::Endpoint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// List of services associated with this schema.
    pub connectivity_service: ServiceResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    /// Unique identifier for the service.
    pub uuid: String,

    /// Name for the service.
    pub value_name: String,

    /// List of nodes associated with this service.
    pub nodes: Vec<NodeResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResponse {
    /// Unique identifier for the node.
    pub node_uuid: String,

    /// Name for the node.
    pub value_name: String,

    /// List of inventories associated with this node.
    pub inventories: Vec<Inventory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    /// Unique identifier for the inventory.
    pub inventory_id: String,

    /// List of endpoints associated with this inventory.
    pub endpoints: Vec<Endpoint>,
}
