use serde_json::Value;

use super::endpoint::BaseEndpoint;

#[derive(Debug, Clone)]
pub struct Link {
    pub link_uuid: String,
    pub node_edge_points: Vec<NodeEdgePoint>,
}

#[derive(Debug, Clone)]
pub struct NodeEdgePoint {
    pub node_edge_point_uuid: String,
    pub node_uuid: String,
}


impl Link {
    pub fn provide_link(&self, base_endpoint: &mut BaseEndpoint) -> Vec<BaseEndpoint> {
        let mut base_endpoint_vector = Vec::new();
    
        // Verifica si alguno de los puntos coincide con el base_endpoint
        if self.node_edge_points.iter().any(|node_edge_point| {
            node_edge_point.node_edge_point_uuid == base_endpoint.node_edge_point_uuid
        }) {
            base_endpoint.link_uuid = Some(self.link_uuid.clone());
    
            // Genera un nuevo ID basado en el existente
            let possible_id = base_endpoint.id.map(|id| id + 1);
    
            // Filtra y genera nuevos BaseEndpoint en una sola operación
            base_endpoint_vector = self
                .node_edge_points
                .iter()
                .filter(|node_edge_point| {
                    node_edge_point.node_edge_point_uuid != base_endpoint.node_edge_point_uuid
                })
                .map(|node_edge_point| BaseEndpoint {
                    node_edge_point_uuid: node_edge_point.node_edge_point_uuid.clone(),
                    node_uuid: node_edge_point.node_uuid.clone(),
                    connection_end_point_uuid: None,
                    service_interface_point_uuid: None,
                    connection_uuid: None,
                    client_node_edge_point_uuid: None,
                    lower_connection: None,
                    link_uuid: Some(self.link_uuid.clone()),
                    inventory_id: None,
                    layer_protocol_qualifier: None,
                    id: possible_id,
                })
                .collect();
        }
    
        base_endpoint_vector
    }
}

pub fn link_vector_build(topology_json: &Value) -> Vec<Link>{

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

    let link_section = topology_object[&link_key_name].clone().as_array().unwrap_or(&Vec::default()).clone();

    let mut link_vector: Vec<Link> = Vec::new();
    
    for link_item in link_section {
        let mut node_edge_point_vector: Vec<NodeEdgePoint> = Vec::new();

        if let Some(node_edge_point_list) = link_item.get("node-edge-point").and_then(Value::as_array) {
            for node_edge_point_item in node_edge_point_list {
                node_edge_point_vector.push(
                    NodeEdgePoint {
                        node_edge_point_uuid: node_edge_point_item.get("node-edge-point-uuid").unwrap_or(&Value::default()).to_string(),
                        node_uuid: node_edge_point_item.get("node-uuid").unwrap_or(&Value::default()).to_string(),
                    }
                );
            }
        }

        link_vector.push(
            Link {
                link_uuid: link_item.get("uuid").unwrap_or(&Value::default()).to_string(),
                node_edge_points: node_edge_point_vector,
            }
        );
    }

    link_vector
}