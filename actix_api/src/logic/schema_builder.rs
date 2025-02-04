use serde_json::{json, Value};

use super::endpoint_builder::build_endpoint_vector;
use crate::{
    models::{
        connections::Connection,
        connectivity_services::Service,
        endpoint::Endpoint,
        links::Link,
        nodes::Node,
        schema::{Inventory, NodeResponse, Schema, ServiceResponse},
    },
    AppError,
};

/// Builds the schema for a connectivity service, including its nodes, inventories, and endpoints.
///
/// # Arguments
/// - `service`: A reference to the `Service` object representing the connectivity service.
/// - `link_vector`: A reference to a vector of `Link` objects representing links in the topology.
/// - `node_vector`: A reference to a vector of `Node` objects representing nodes in the topology.
/// - `connection_vector`: A reference to a vector of `Connection` objects representing connections.
///
/// # Returns
/// A `Result` containing a serialized JSON value representing the schema or an `aPPError`.
pub fn build_schema(
    service: &Service,
    link_vector: &Vec<Link>,
    node_vector: &Vec<Node>,
    connection_vector: &Vec<Connection>,
) -> Result<Value, AppError> {
    let mut node_response_vector: Vec<NodeResponse> = Vec::new();
    let mut inventories_response_vector: Vec<Inventory> = Vec::new();

    let endpoint_vector =
        build_endpoint_vector(service, link_vector, node_vector, connection_vector);

    for endpoint in endpoint_vector {
        // Create inventories
        if !inventories_response_vector
            .iter()
            .any(|inventory| inventory.inventory_id == endpoint.inventory_id)
        {
            let inventory_endpoint_vector: Vec<Endpoint> = vec![endpoint.clone()];
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
        // Create nodes
        if !node_response_vector
            .iter()
            .any(|node| node.node_uuid == inventory.endpoints[0].node_uuid)
        {
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

    // Alternated obtain sorted nodes
    let mut node_with_min_ids: Vec<(NodeResponse, i32)> = node_response_vector
        .iter()
        .filter_map(|node| {
            let min_id = node
                .inventories
                .iter()
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

    right.reverse(); // The right side have to keep it reverse.

    let total_nodes = left.len() + right.len();
    let middle_index = total_nodes / 2;

    // Nodes processing
    for (i, node) in left.iter_mut().enumerate() {
        if i == middle_index && total_nodes % 2 != 0 {
            // Middle Node
            node.inventories = sort_middle_inventories(node.inventories.clone());
            reordered_nodes.push(node.clone());
        } else {
            node.inventories
                .sort_by_key(|inv| inv.endpoints.iter().map(|e| e.id).min().unwrap_or(i32::MAX));
            reordered_nodes.push(node.clone());
        }
    }

    for node in right.iter_mut() {
        node.inventories.sort_by_key(|inv| {
            std::cmp::Reverse(inv.endpoints.iter().map(|e| e.id).min().unwrap_or(i32::MAX))
        });
        reordered_nodes.push(node.clone());
    }

    // Build final response
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

/// Sorts inventories in a zig-zag pattern for the middle node.
///
/// # Arguments
/// - `inventories`: A vector of `Inventory` objects to be sorted.
///
/// # Returns
/// A vector of `Inventory` objects sorted alternately (zig-zag pattern).
///
/// # Process
/// - Sorts inventories by the smallest endpoint ID in ascending order.
/// - Alternates inventories between the left and right sides.
/// - Returns the reordered inventories as a single vector.
///
/// # Notes
/// - This function is used specifically for the middle node in the schema to achieve
///   an alternate ordering effect.
fn sort_middle_inventories(mut inventories: Vec<Inventory>) -> Vec<Inventory> {
    inventories.sort_by_key(|inv| inv.endpoints.iter().map(|e| e.id).min().unwrap_or(i32::MAX));

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

    result.into_iter().flatten().collect()
}
