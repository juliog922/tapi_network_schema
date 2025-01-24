use serde::{Deserialize, Serialize};

use super::endpoint::Endpoint;

/// Represents the overall schema, including a connectivity service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub connectivity_service: ServiceResponse,
}

/// Represents a service and its associated nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    pub uuid: String,
    pub value_name: String,
    pub nodes: Vec<NodeResponse>,
}

/// Represents a node and its associated inventories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResponse {
    pub node_uuid: String,
    pub value_name: String,
    pub inventories: Vec<Inventory>,
}

/// Represents an inventory with associated endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub inventory_id: String,
    pub endpoints: Vec<Endpoint>,
}
