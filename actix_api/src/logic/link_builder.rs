use serde_json::Value;

use crate::models::links::{Link, NodeEdgePoint};

/// Builds a vector of `Link` objects from the provided JSON topology.
///
/// # Arguments
///
/// * `topology_json` - A reference to a JSON value representing the topology.
///
/// # Returns
///
/// A vector of `Link` objects constructed from the topology JSON.
pub fn link_vector_build(topology_json: &Value) -> Vec<Link> {
    // Determine the topology object, supporting both array and object JSON structures.
    let topology_object = if let Some(topology_vec) = topology_json.as_array() {
        topology_vec[0].as_object().unwrap()
    } else {
        topology_json.as_object().unwrap()
    };

    let mut link_key_name = String::default();

    // Search for a key that contains "link" within the topology object.
    'link_search: for (key, _) in topology_object {
        if key.contains("link") {
            link_key_name = key.clone();
            break 'link_search;
        }
    }

    // Extract the link section as an array, defaulting to an empty array if missing.
    let link_section = topology_object[&link_key_name]
        .clone()
        .as_array()
        .unwrap_or(&Vec::default())
        .clone();

    let mut link_vector: Vec<Link> = Vec::new();

    // Process each link item in the link section.
    for link_item in link_section {
        let mut node_edge_point_vector: Vec<NodeEdgePoint> = Vec::new();

        // Extract and process the "node-edge-point" section if present.
        if let Some(node_edge_point_list) =
            link_item.get("node-edge-point").and_then(Value::as_array)
        {
            for node_edge_point_item in node_edge_point_list {
                node_edge_point_vector.push(NodeEdgePoint {
                    node_edge_point_uuid: node_edge_point_item
                        .get("node-edge-point-uuid")
                        .unwrap_or(&Value::default())
                        .to_string(),
                    node_uuid: node_edge_point_item
                        .get("node-uuid")
                        .unwrap_or(&Value::default())
                        .to_string(),
                });
            }
        }

        // Add the constructed `Link` object to the vector.
        link_vector.push(Link {
            link_uuid: link_item
                .get("uuid")
                .unwrap_or(&Value::default())
                .to_string(),
            node_edge_points: node_edge_point_vector,
        });
    }

    link_vector
}
