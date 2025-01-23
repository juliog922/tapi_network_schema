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
