use serde_json::{Value, Map};

/// Represents an endpoint in a network schema.
///
/// This structure contains various identifiers and optional attributes related to an endpoint
/// in a network. Each field is represented as a `serde_json::Value`, which allows for flexible 
/// data handling and serialization.
#[derive(Debug, Clone)]
pub struct Endpoint {
    /// UUID of the connection endpoint.
    pub connection_end_point_uuid: Value,

    /// UUID of the node edge point.
    pub node_edge_point_uuid: Value,

    /// INVENTORY ID (card id)
    pub inventory_id: Value,

    /// Layer protocol qualifier associated with this endpoint.
    pub layer_protocol_qualifier: Value,

    /// Optional UUID of the client node edge point.
    pub client_node_edge_point_uuid: Option<Value>,

    /// Optional UUID of the service interface point.
    pub service_interface_point_uuid: Option<Value>,

    /// Optional field representing lower connections related to this endpoint.
    pub lower_connections: Option<Value>,

    /// Optional UUID of the link associated with this endpoint.
    pub link_uuid: Option<Value>,

    /// Optional UUID of the connection associated with this endpoint.
    pub connection_uuid: Option<Value>,

    /// Unique identifier for the endpoint.
    pub id: Value,
}

impl Endpoint {
    /// Constructs a new `Endpoint` with the provided values.
    ///
    /// # Parameters
    /// - `connection_end_point_uuid`: UUID for the connection endpoint.
    /// - `node_edge_point_uuid`: UUID for the node edge point.
    /// - `layer_protocol_qualifier`: Layer protocol qualifier for this endpoint.
    /// - `client_node_edge_point_uuid`: Optional UUID for the client node edge point.
    /// - `service_interface_point_uuid`: Optional UUID for the service interface point.
    /// - `lower_connections`: Optional field for lower connections.
    /// - `link_uuid`: Optional UUID for the link.
    /// - `id`: Unique identifier for the endpoint.
    ///
    /// # Returns
    /// A new instance of `Endpoint`.
    pub fn new(
        connection_end_point_uuid: Value,
        node_edge_point_uuid: Value,
        inventory_id: Value,
        layer_protocol_qualifier: Value,
        client_node_edge_point_uuid: Option<Value>,
        service_interface_point_uuid: Option<Value>,
        lower_connections: Option<Value>,
        link_uuid: Option<Value>,
        connection_uuid: Option<Value>,
        id: Value,
    ) -> Self {
        Endpoint {
            connection_end_point_uuid,
            node_edge_point_uuid,
            inventory_id,
            layer_protocol_qualifier,
            client_node_edge_point_uuid,
            service_interface_point_uuid,
            lower_connections,
            link_uuid,
            connection_uuid,
            id,
        }
    }

    /// Converts the `Endpoint` instance into a `serde_json::Value`.
    ///
    /// This method creates a JSON object representing the endpoint, including all its fields.
    /// Optional fields are only included if they are present.
    ///
    /// # Returns
    /// A `serde_json::Value` representing the `Endpoint` as a JSON object.
    pub fn to_value(&self) -> Value {
        // Create a new JSON object to hold the endpoint data
        let mut endpoint_obj = Value::Object(Map::new());

        // Insert required fields into the JSON object
        endpoint_obj.as_object_mut().unwrap().insert("connection_end_point_uuid".to_string(), self.connection_end_point_uuid.clone());
        endpoint_obj.as_object_mut().unwrap().insert("node_edge_point_uuid".to_string(), self.node_edge_point_uuid.clone());
        endpoint_obj.as_object_mut().unwrap().insert("inventory_id".to_string(), self.inventory_id.clone());
        endpoint_obj.as_object_mut().unwrap().insert("layer_protocol_qualifier".to_string(), self.layer_protocol_qualifier.clone());

        // Conditionally insert optional fields into the JSON object if they are present
        if let Some(ref client_node_edge_point_uuid) = self.client_node_edge_point_uuid {
            endpoint_obj.as_object_mut().unwrap().insert("client_node_edge_point_uuid".to_string(), client_node_edge_point_uuid.clone());
        }

        if let Some(ref service_interface_point_uuid) = self.service_interface_point_uuid {
            endpoint_obj.as_object_mut().unwrap().insert("service_interface_point_uuid".to_string(), service_interface_point_uuid.clone());
        }
        
        if let Some(ref lower_connections) = self.lower_connections {
            endpoint_obj.as_object_mut().unwrap().insert("lower_connections".to_string(), lower_connections.clone());
        }

        if let Some(ref link_uuid) = self.link_uuid {
            endpoint_obj.as_object_mut().unwrap().insert("link_uuid".to_string(), link_uuid.clone());
        }

        if let Some(ref connection_uuid) = self.connection_uuid {
            endpoint_obj.as_object_mut().unwrap().insert("connection_uuid".to_string(), connection_uuid.clone());
        }

        // Insert the unique identifier for the endpoint
        endpoint_obj.as_object_mut().unwrap().insert("id".to_string(), self.id.clone());

        endpoint_obj
    }
}
