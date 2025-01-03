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
    pub connectivity_service: ServiceResponse
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
    service: &Service,
    link_vector: &Vec<Link>,
    node_vector: &Vec<Node>,
    connection_vector: &Vec<Connection>,
) -> Result<Value, Error> {

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

    // Obtener nodos ordenados alternadamente
    let mut node_with_min_ids: Vec<(NodeResponse, i32)> = node_response_vector
        .iter()
        .filter_map(|node| {
            let min_id = node.inventories.iter()
                .flat_map(|inv| inv.endpoints.iter().map(|e| e.id))
                .min();
            min_id.map(|id| (node.clone(), id))
        })
        .collect();

    node_with_min_ids.sort_by(|a, b| a.1.cmp(&b.1));

    let mut reordered_nodes: Vec<NodeResponse> = Vec::new();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for (i, (node, _)) in node_with_min_ids.iter().enumerate() {
        if i % 2 == 0 {
            left.push(node.clone());
        } else {
            right.push(node.clone());
        }
    }

    right.reverse(); // El lado derecho debe mantenerse al revés

    let total_nodes = left.len() + right.len();
    let middle_index = total_nodes / 2;

    // Procesar nodos alternadamente
    for (i, node) in left.iter_mut().enumerate() {
        if i == middle_index && total_nodes % 2 != 0 {
            // Nodo del medio
            node.inventories = sort_middle_inventories(node.inventories.clone());
            reordered_nodes.push(node.clone());
        } else {
            node.inventories.sort_by_key(|inv| {
                inv.endpoints.iter().map(|e| e.id).min().unwrap_or(i32::MAX)
            });
            reordered_nodes.push(node.clone());
        }
    }

    for node in right.iter_mut() {
        node.inventories.sort_by_key(|inv| {
            std::cmp::Reverse(inv.endpoints.iter().map(|e| e.id).min().unwrap_or(i32::MAX))
        });
        reordered_nodes.push(node.clone());
    }

    // Construir la respuesta final
    let service_response = ServiceResponse {
        uuid: service.service_uuid.clone(),
        value_name: service.name.clone(),
        nodes: reordered_nodes,
    };

    let schema = Schema {
        connectivity_service: service_response,
    };

    Ok(json!(schema.connectivity_service))
}

/// Función que ordena los inventarios en zig-zag para el nodo del medio
fn sort_middle_inventories(mut inventories: Vec<Inventory>) -> Vec<Inventory> {
    inventories.sort_by_key(|inv| {
        inv.endpoints.iter().map(|e| e.id).min().unwrap_or(i32::MAX)
    });

    let mut result = vec![None; inventories.len()];
    let mut left = 0;
    let mut right = inventories.len() - 1;
    let mut toggle = true;

    for inventory in inventories {
        if toggle {
            result[left] = Some(inventory);
            left += 1;
        } else {
            result[right] = Some(inventory);
            right -= 1;
        }
        toggle = !toggle;
    }

    result.into_iter().filter_map(|inv| inv).collect()
}
