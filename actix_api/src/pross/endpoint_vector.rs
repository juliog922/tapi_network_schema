use super::connections::Connection;
use super::endpoint::{BaseEndpoint, Endpoint};
use super::connectivity_services::Service;
use super::links::Link;
use super::nodes::Node;

pub fn build_endpoint_vector(
    service: &Service,
    link_vector: &Vec<Link>,
    node_vector: &Vec<Node>,
    connection_vector: &Vec<Connection>,
) -> Vec<Endpoint> {
    let mut endpoint_vector: Vec<Endpoint> = Vec::new();
    let mut base_endpoint_vector: Vec<BaseEndpoint> = service.first_base_endpoint_vector();
    let mut processed_node_edge_uuids: Vec<String> = Vec::new(); // Usamos Vec para rastrear UUIDs procesados.

    while let Some(base_endpoint) = base_endpoint_vector.pop() {
        // Solo procesar si no se ha visto antes
        if !processed_node_edge_uuids.contains(&base_endpoint.node_edge_point_uuid) {
            processed_node_edge_uuids.push(base_endpoint.node_edge_point_uuid.clone()); // Añadir UUID a la lista de procesados
            let (endpoint, extend_base_endpoint_vector) =
                base_endpoint.build(link_vector, node_vector, connection_vector);
            if let Some(endpoint_connection_uuid) = endpoint.connection_uuid.clone() {
                
                if service.connections.iter().any(|connection| {
                    connection.connection_uuid == endpoint_connection_uuid
                }) | service.lower_connections.iter().any(|lower_connection| {
                    lower_connection.connection_uuid == endpoint_connection_uuid
                }) {
                    endpoint_vector.push(endpoint);
                } else {
                    println!("This connection its broken: {}", endpoint_connection_uuid);
                }
            } else {
                println!("This endpoint node_edge_point_uuid dont have connection uuid: {}", endpoint.node_edge_point_uuid);
                endpoint_vector.push(endpoint);
            }
            
            base_endpoint_vector.extend(extend_base_endpoint_vector);
        }
    }

    endpoint_vector
}
