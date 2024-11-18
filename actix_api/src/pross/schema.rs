use actix_web::Error;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};

use super::connections::Connection;
use super::endpoint::Endpoint;
use super::connectivity_services::Service;
use super::links::Link;
use super::nodes::Node;
use super::endpoint_vector::build_endpoint_vector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {

    /// List of services associated with this schema.
    pub connectivity_services: Vec<ServiceResponse>
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
    pub endpoints: Vec<Endpoint>
}


pub fn build_schema(
    service_vector: &Vec<Service>,
    link_vector: &Vec<Link>,
    node_vector: &Vec<Node>,
    connection_vector: &Vec<Connection>,
) -> Result<Value, Error> {
    let mut schema = Schema {
        connectivity_services: Vec::new(),
    };

    for service in service_vector {
        let mut node_response_vector: Vec<NodeResponse> = Vec::new();
        let mut inventories_response_vector: Vec<Inventory> = Vec::new();

        let endpoint_vector = build_endpoint_vector(service, link_vector, node_vector, connection_vector);

        for endpoint in endpoint_vector {
            // Crear inventarios
            if !inventories_response_vector.iter().any(|inventory| inventory.inventory_id == endpoint.inventory_id) {
                let mut inventory_endpoint_vector: Vec<Endpoint> = Vec::new();
                inventory_endpoint_vector.push(endpoint.clone());
                inventories_response_vector.push(Inventory {
                    inventory_id: endpoint.inventory_id.clone(),
                    endpoints: inventory_endpoint_vector,
                });
            } else {
                inventories_response_vector
                    .iter_mut()
                    .filter(|inventory| inventory.inventory_id == endpoint.inventory_id)
                    .for_each(|inventory| {
                        inventory.endpoints.push(endpoint.clone());
                    });
            }
        }

        for inventory in &inventories_response_vector {
            // Crear nodos
            if !node_response_vector.iter().any(|node| {
                node.node_uuid == inventory.endpoints[0].node_uuid
            }) {
                node_response_vector.push(NodeResponse {
                    node_uuid: inventory.endpoints[0].node_uuid.clone(),
                    value_name: node_vector
                        .iter()
                        .find(|node| node.node_uuid == inventory.endpoints[0].node_uuid)
                        .map_or("Unknown".to_string(), |node| node.name.clone()),
                    inventories: vec![inventory.clone()],
                });
            } else {
                node_response_vector
                    .iter_mut()
                    .filter(|node| node.node_uuid == inventory.endpoints[0].node_uuid)
                    .for_each(|node| {
                        node.inventories.push(inventory.clone());
                    });
            }
        }

        let service_response = ServiceResponse {
            uuid: service.service_uuid.clone(),
            value_name: service.name.clone(),
            nodes: node_response_vector,
        };

        schema.connectivity_services.push(service_response);
    }

    Ok(json!(schema.connectivity_services))
}