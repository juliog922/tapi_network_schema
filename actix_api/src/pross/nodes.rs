use serde_json::Value;

use super::{connections, endpoint::BaseEndpoint};
use crate::utils::find_name;

#[derive(Debug, Clone)]
pub struct Node {
    pub node_uuid: String,
    pub name: String,
    pub owned_node_edge_points: Vec<OwnedNodeEdgePoint>,
}

#[derive(Debug, Clone)]
pub struct OwnedNodeEdgePoint {
    pub node_edge_point_uuid: String,
    pub inventory_id: String,
    pub connection_end_points: Vec<NodeConnectionEndPoint>,
}

#[derive(Debug, Clone)]
pub struct NodeConnectionEndPoint {
    pub connection_end_point_uuid: String,
    pub layer_protocol_qualifier: String,
    pub client_node_edge_points: Vec<ClientNodeEdgePoint>,
}

#[derive(Debug, Clone)]
pub struct ClientNodeEdgePoint {
    pub node_edge_point_uuid: String,
    pub node_uuid: String,
}

impl Node {
    pub fn provide_inventory(&self, base_endpoint: &mut BaseEndpoint) -> Vec<BaseEndpoint> {
        let mut base_endpoint_vector = Vec::new();

        if self.node_uuid == base_endpoint.node_uuid {
            for owned_node_edge_point in &self.owned_node_edge_points {
                if owned_node_edge_point.node_edge_point_uuid == base_endpoint.node_edge_point_uuid {

                    base_endpoint.inventory_id = Some(owned_node_edge_point.inventory_id.clone());

                    for connection_end_point in owned_node_edge_point.connection_end_points.iter(){
                        if base_endpoint.connection_end_point_uuid.is_none() || base_endpoint.connection_end_point_uuid.clone().unwrap() == connection_end_point.connection_end_point_uuid {
                                
                            base_endpoint.layer_protocol_qualifier = Some(connection_end_point.layer_protocol_qualifier.clone());
                            base_endpoint.connection_end_point_uuid = Some(connection_end_point.connection_end_point_uuid.clone());

                            if connection_end_point.client_node_edge_points.len() > 1 {
                                println!("There is more than 2 Clients for the same Endpoint for node-edge-point-uuid: {}", base_endpoint.node_edge_point_uuid);
                            } 

                            if !connection_end_point.client_node_edge_points.is_empty() {
                                base_endpoint.client_node_edge_point_uuid = Some(connection_end_point.client_node_edge_points[0].node_edge_point_uuid.clone());
    
                                let possible_id = base_endpoint.id.map(|id| id + 1);
    
                                base_endpoint_vector.push(
                                    BaseEndpoint {
                                        node_edge_point_uuid: connection_end_point.client_node_edge_points[0].node_edge_point_uuid.clone(),
                                        node_uuid: connection_end_point.client_node_edge_points[0].node_uuid.clone(),
                                        connection_end_point_uuid: None,
                                        service_interface_point_uuid: None,
                                        connection_uuid: None,
                                        client_node_edge_point_uuid: None,
                                        lower_connection: None,
                                        link_uuid: None,
                                        layer_protocol_qualifier: None,
                                        inventory_id: None,
                                        id: possible_id,
                                    }
                                );
                            }
                        }
                    }

                }
            }
             
        }
        // Find its Parent to add to
        for owned_node_edge_point in &self.owned_node_edge_points {
            if owned_node_edge_point.connection_end_points.len() > 0 && owned_node_edge_point.connection_end_points[0].client_node_edge_points.len() > 0 {
                if owned_node_edge_point.connection_end_points[0].client_node_edge_points[0].node_edge_point_uuid == base_endpoint.node_edge_point_uuid {
                    let possible_id = base_endpoint.id.map(|id| id - 1);
                    base_endpoint_vector.push(
                        BaseEndpoint {
                            node_edge_point_uuid: owned_node_edge_point.node_edge_point_uuid.clone(),
                            node_uuid: self.node_uuid.clone(),
                            connection_end_point_uuid: None,
                            service_interface_point_uuid: None,
                            connection_uuid: None,
                            client_node_edge_point_uuid: None,
                            lower_connection: None,
                            link_uuid: None,
                            layer_protocol_qualifier: None,
                            inventory_id: None,
                            id: possible_id,
                        }
                    );
                }
            }

        }

        base_endpoint_vector
    }
}


pub fn node_vector_building(topology_json: &Value) -> Vec<Node> {

    let topology_object = if let Some(topology_vec) = topology_json.as_array() {
        topology_vec[0].as_object().unwrap()
    } else {
        topology_json.as_object().unwrap()
    };

    let mut node_key_name = String::default();

    'node_search: for (key, _) in topology_object {
        if key.contains("node") {
            node_key_name = key.clone();
            break 'node_search;
        }
    }

    let node_section = topology_object[&node_key_name].clone().as_array().unwrap_or(&Vec::default()).clone();

    let mut node_vector: Vec<Node> = Vec::new();

    for node_item in node_section {

        let mut owned_node_edge_point_vector: Vec<OwnedNodeEdgePoint> = Vec::new();

        if let Some(owned_node_edge_point_section) = node_item.get("owned-node-edge-point").and_then(Value::as_array) {
            for owned_node_edge_point_item in owned_node_edge_point_section {

                let mut connection_end_point_vector: Vec<NodeConnectionEndPoint> = Vec::new();

                if let Some(cep_list_section) = owned_node_edge_point_item.get("tapi-connectivity:cep-list") {
                    if let Some(connection_end_point_section) = cep_list_section.get("connection-end-point").and_then(Value::as_array) {
                        for connection_end_point_item  in connection_end_point_section {

                            let mut client_node_edge_point_vector: Vec<ClientNodeEdgePoint> = Vec::new();

                            if let Some(client_node_edge_point_section) = connection_end_point_item.get("client-node-edge-point").and_then(Value::as_array) {
                                for client_node_edge_point_item in client_node_edge_point_section {

                                    client_node_edge_point_vector.push(
                                        ClientNodeEdgePoint {
                                            node_edge_point_uuid: client_node_edge_point_item.get("node-edge-point-uuid").unwrap_or(&Value::default()).to_string(),
                                            node_uuid: client_node_edge_point_item.get("node-uuid").unwrap_or(&Value::default()).to_string(),
                                        }
                                    );

                                }
                            }

                            connection_end_point_vector.push(
                                NodeConnectionEndPoint {
                                    connection_end_point_uuid: connection_end_point_item.get("uuid").unwrap_or(&Value::default()).to_string(),
                                    layer_protocol_qualifier: connection_end_point_item.get("layer-protocol-qualifier").unwrap_or(&Value::default()).to_string(),
                                    client_node_edge_points: client_node_edge_point_vector,
                                }
                            );

                        }

                    }
                }

                owned_node_edge_point_vector.push(
                    OwnedNodeEdgePoint {
                        node_edge_point_uuid: owned_node_edge_point_item.get("uuid").unwrap_or(&Value::default()).to_string(),
                        inventory_id: find_name(owned_node_edge_point_item, "INVENTORY_ID".to_string()),
                        connection_end_points: connection_end_point_vector,
                    }
                );
            }

        }

        node_vector.push(
            Node {
                node_uuid: node_item.get("uuid").unwrap_or(&Value::default()).to_string(),
                name: find_name(&node_item, "NODE_IDENTIFIER".to_string()), ////////////
                owned_node_edge_points: owned_node_edge_point_vector,
            }
        );
    }

    node_vector
}