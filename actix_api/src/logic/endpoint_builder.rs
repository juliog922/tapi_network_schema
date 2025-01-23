use crate::models::{
    connections::Connection,
    connectivity_services::Service,
    endpoint::{BaseEndpoint, Endpoint},
    links::Link,
    nodes::Node,
};

/// Builds a vector of `Endpoint` objects based on the provided service and related entities.
///
/// # Arguments
///
/// * `service` - A reference to the `Service` containing base endpoint data.
/// * `link_vector` - A reference to a vector of `Link` objects.
/// * `node_vector` - A reference to a vector of `Node` objects.
/// * `connection_vector` - A reference to a vector of `Connection` objects.
///
/// # Returns
///
/// A vector of `Endpoint` objects constructed from the provided data.
pub fn build_endpoint_vector(
    service: &Service,
    link_vector: &Vec<Link>,
    node_vector: &Vec<Node>,
    connection_vector: &Vec<Connection>,
) -> Vec<Endpoint> {
    let mut endpoint_vector: Vec<Endpoint> = Vec::new();
    let mut base_endpoint_vector: Vec<BaseEndpoint> = service.first_base_endpoint_vector();
    let mut processed_node_edge_uuids: Vec<String> = Vec::new(); // It will use to keep track of processed UUID's.

    while let Some(base_endpoint) = base_endpoint_vector.pop() {
        // Skip already processed UUIDs
        if !processed_node_edge_uuids.contains(&base_endpoint.node_edge_point_uuid) {
            processed_node_edge_uuids.push(base_endpoint.node_edge_point_uuid.clone()); // Añadir UUID a la lista de procesados
            let (endpoint, extend_base_endpoint_vector) =
                base_endpoint.build(link_vector, node_vector, connection_vector);

            if let Some(endpoint_connection_uuid) = endpoint.connection_uuid.clone() {
                // Verify if the connection belongs to the current service
                if service
                    .connections
                    .iter()
                    .any(|connection| connection.connection_uuid == endpoint_connection_uuid)
                    | service.lower_connections.iter().any(|lower_connection| {
                        lower_connection.connection_uuid == endpoint_connection_uuid
                    })
                {
                    endpoint_vector.push(endpoint);
                    base_endpoint_vector.splice(0..0, extend_base_endpoint_vector);
                } else {
                    println!(
                        "This connection belongs to other service: {}",
                        endpoint_connection_uuid
                    );
                }
            } else {
                println!(
                    "This endpoint node_edge_point_uuid dont have connection uuid: {}",
                    endpoint.node_edge_point_uuid
                );
                endpoint_vector.push(endpoint);
                base_endpoint_vector.splice(0..0, extend_base_endpoint_vector);
            }
        }
    }

    endpoint_vector
}
