use serde_json::Value;

use crate::{
    models::nodes::{ClientNodeEdgePoint, Node, NodeConnectionEndPoint, OwnedNodeEdgePoint},
    utils::find_name,
};

/// Builds a vector of `Node` objects from the provided JSON topology.
///
/// # Arguments
///
/// * `topology_json` - A reference to a JSON value representing the topology.
///
/// # Returns
///
/// A vector of `Node` objects constructed from the topology JSON.
pub fn node_vector_building(topology_json: &Value) -> Vec<Node> {
    // Determine the topology object, supporting both array and object JSON structures.
    let topology_object = if let Some(topology_vec) = topology_json.as_array() {
        topology_vec[0].as_object().unwrap()
    } else {
        topology_json.as_object().unwrap()
    };

    let mut node_key_name = String::default();

    // Search for a key that contains "node" in the topology object.
    'node_search: for (key, _) in topology_object {
        if key.contains("node") {
            node_key_name = key.clone();
            break 'node_search;
        }
    }

    // Extract the node section as an array, defaulting to an empty array if missing.
    let node_section = topology_object[&node_key_name]
        .clone()
        .as_array()
        .unwrap_or(&Vec::default())
        .clone();

    let mut node_vector: Vec<Node> = Vec::new();

    // Process each node item in the node section.
    for node_item in node_section {
        let mut owned_node_edge_point_vector: Vec<OwnedNodeEdgePoint> = Vec::new();

        if let Some(owned_node_edge_point_section) = node_item
            .get("owned-node-edge-point")
            .and_then(Value::as_array)
        {
            for owned_node_edge_point_item in owned_node_edge_point_section {
                let mut connection_end_point_vector: Vec<NodeConnectionEndPoint> = Vec::new();

                if let Some(cep_list_section) =
                    owned_node_edge_point_item.get("tapi-connectivity:cep-list")
                {
                    if let Some(connection_end_point_section) = cep_list_section
                        .get("connection-end-point")
                        .and_then(Value::as_array)
                    {
                        for connection_end_point_item in connection_end_point_section {
                            let mut client_node_edge_point_vector: Vec<ClientNodeEdgePoint> =
                                Vec::new();

                            if let Some(client_node_edge_point_section) = connection_end_point_item
                                .get("client-node-edge-point")
                                .and_then(Value::as_array)
                            {
                                for client_node_edge_point_item in client_node_edge_point_section {
                                    client_node_edge_point_vector.push(ClientNodeEdgePoint {
                                        node_edge_point_uuid: client_node_edge_point_item
                                            .get("node-edge-point-uuid")
                                            .unwrap_or(&Value::default())
                                            .to_string(),
                                        node_uuid: client_node_edge_point_item
                                            .get("node-uuid")
                                            .unwrap_or(&Value::default())
                                            .to_string(),
                                    });
                                }
                            }

                            connection_end_point_vector.push(NodeConnectionEndPoint {
                                connection_end_point_uuid: connection_end_point_item
                                    .get("uuid")
                                    .unwrap_or(&Value::default())
                                    .to_string(),
                                layer_protocol_qualifier: connection_end_point_item
                                    .get("layer-protocol-qualifier")
                                    .unwrap_or(&Value::default())
                                    .to_string(),
                                client_node_edge_points: client_node_edge_point_vector,
                            });
                        }
                    }
                }

                owned_node_edge_point_vector.push(OwnedNodeEdgePoint {
                    node_edge_point_uuid: owned_node_edge_point_item
                        .get("uuid")
                        .unwrap_or(&Value::default())
                        .to_string(),
                    inventory_id: find_name(owned_node_edge_point_item, "INVENTORY_ID".to_string()),
                    connection_end_points: connection_end_point_vector,
                });
            }
        }

        node_vector.push(Node {
            node_uuid: node_item
                .get("uuid")
                .unwrap_or(&Value::default())
                .to_string(),
            name: find_name(&node_item, "NODE_IDENTIFIER".to_string()), ////////////
            owned_node_edge_points: owned_node_edge_point_vector,
        });
    }

    node_vector
}
