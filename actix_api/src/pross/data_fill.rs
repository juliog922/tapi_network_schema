use serde_json::Value;
use std::collections::HashMap;



pub fn link_mapping(topology_json: &Value) -> HashMap<String, Value> {

    let topology_object = if let Some(topology_vec) = topology_json.as_array() {
        topology_vec[0].as_object().unwrap()
    } else {
        topology_json.as_object().unwrap()
    };

    let mut link_key_name = String::default();

    'link_search: for (key, _) in topology_object {
        if key.contains("link") {
            link_key_name = key.clone();
            break 'link_search;
        }
    }

    let link_sections = topology_object[&link_key_name].clone().as_array().unwrap().clone();

    let mut link_nepu_hashmap: HashMap<String, Value> = HashMap::new();

    for link_section in link_sections {
        if let Some(link_uuid) = link_section.get("uuid") {

            if let Some(node_edge_point_list) = link_section.get("node-edge-point").and_then(Value::as_array) {

                for node_edge_point in node_edge_point_list {

                    if let Some(node_edge_point_uuid) = node_edge_point.get("node-edge-point-uuid") {
                        link_nepu_hashmap.insert(
                            node_edge_point_uuid.to_string(), 
                            link_uuid.clone()
                        );
                    }

                }

            }
        }
    }

    link_nepu_hashmap
}