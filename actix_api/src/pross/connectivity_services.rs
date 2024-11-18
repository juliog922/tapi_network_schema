use serde_json::Value;

use super::endpoint::BaseEndpoint;
use crate::utils::find_name;

#[derive(Debug, Clone)]
pub struct Service {
    pub service_uuid: String,
    pub name: String,
    pub end_points: Vec<EndPoint>,
}

#[derive(Debug, Clone)]
pub struct EndPoint {
    pub name: String,
    pub location: String,
    pub connection_end_points: Vec<ServiceConnectionEndPoint>,
    pub service_interface_point_uuid: String,
}

#[derive(Debug, Clone)]
pub struct ServiceConnectionEndPoint {
    pub node_edge_point_uuid: String,
    pub connection_end_point_uuid: String,
    pub node_uuid: String,
}

impl Service {
    pub fn first_base_endpoint_vector(&self) -> Vec<BaseEndpoint> {
        let mut base_endpoint_vector = Vec::new();

        self.end_points.iter().for_each(|end_point| {
            end_point.connection_end_points.iter().for_each(|connection_end_point| {
                base_endpoint_vector.push(
                    BaseEndpoint {
                        node_edge_point_uuid: connection_end_point.node_edge_point_uuid.clone(),
                        node_uuid: connection_end_point.node_uuid.clone(),
                        connection_end_point_uuid: Some(connection_end_point.connection_end_point_uuid.clone()),
                        service_interface_point_uuid: Some(end_point.service_interface_point_uuid.clone()),
                        connection_uuid: None,
                        client_node_edge_point_uuid: None,
                        lower_connection: None,
                        link_uuid: None,
                        layer_protocol_qualifier: None,
                        inventory_id: None,
                        id: Some(1),
                    }
                );
            });
        });
        base_endpoint_vector
    }
}

pub fn connectivity_service_vector_build(connectivity_services_json: &Vec<Value>) -> Vec<Service> {
    
    let mut connectivity_service_vector: Vec<Service> = Vec::new();

    for service_item in connectivity_services_json {

        let mut end_point_vector: Vec<EndPoint> = Vec::new();

        if let Some(end_point_section) = service_item.get("end-point").and_then(Value::as_array) {
            for end_point_item in end_point_section {

                let mut service_connection_end_point_vector: Vec<ServiceConnectionEndPoint> = Vec::new();

                if let Some(connection_end_point_section) = end_point_item.get("connection-end-point").and_then(Value::as_array) {
                    for connection_end_point_item in connection_end_point_section {

                        service_connection_end_point_vector.push(
                            ServiceConnectionEndPoint {
                                node_edge_point_uuid: connection_end_point_item.get("node-edge-point-uuid").unwrap_or(&Value::default()).to_string(),
                                connection_end_point_uuid: connection_end_point_item.get("connection-end-point-uuid").unwrap_or(&Value::default()).to_string(),
                                node_uuid: connection_end_point_item.get("node-uuid").unwrap_or(&Value::default()).to_string(),
                            }
                        );

                    }
                }

                end_point_vector.push(
                    EndPoint {
                        name: find_name(end_point_item, "CSEP_NAME".to_string()),
                        location: find_name(end_point_item, "location".to_string()),
                        connection_end_points: service_connection_end_point_vector,
                        service_interface_point_uuid: end_point_item.get("service-interface-point").unwrap_or(&Value::default())
                                                        .get("service-interface-point-uuid").unwrap_or(&Value::default()).to_string(),
                    }   
                );

            }
        }

        connectivity_service_vector.push(
            Service {
                service_uuid: service_item.get("uuid").unwrap_or(&Value::default()).to_string(),
                name: find_name(service_item, "SERVICE_NAME".to_string()),
                end_points: end_point_vector,
            }
        );
    }

    connectivity_service_vector
}

