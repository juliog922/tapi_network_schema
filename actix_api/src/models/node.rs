use serde_json::{Value, Map};
use crate::models::endpoint::Endpoint;

/// Represents a node in the network schema.
///
/// A `Node` is an entity within the network schema that has a unique identifier and a list of
/// associated endpoints. This structure allows for representing and serializing a node along with
/// its endpoints in a JSON-compatible format.
#[derive(Debug, Clone)]
pub struct Node {
    /// Unique identifier for the node.
    pub node_uuid: String,

    /// List of endpoints associated with this node.
    pub endpoints: Vec<Endpoint>,
}

impl Node {
    /// Constructs a new `Node` with the specified UUID.
    ///
    /// # Parameters
    /// - `node_uuid`: The unique identifier for the node.
    ///
    /// # Returns
    /// A new instance of `Node` with the provided UUID and an empty list of endpoints.
    pub fn new(node_uuid: String) -> Self {
        Node {
            node_uuid,
            endpoints: vec![],
        }
    }

    /// Converts the `Node` instance into a `serde_json::Value`.
    ///
    /// This method creates a JSON object representing the node, including its UUID and a list of
    /// its endpoints. Each endpoint is converted to a JSON value using its `to_value` method.
    ///
    /// # Returns
    /// A `serde_json::Value` representing the `Node` as a JSON object. The object includes:
    /// - `node_uuid`: The UUID of the node.
    /// - `end_points`: A JSON array containing the serialized endpoints.
    pub fn to_value(&self) -> Value {
        // Create a new JSON object to hold the node data
        let mut node_obj = Value::Object(Map::new());

        // Insert the node UUID into the JSON object
        node_obj.as_object_mut().unwrap().insert("node_uuid".to_string(), Value::String(self.node_uuid.clone()));
        
        // Convert the list of endpoints into a JSON array and insert it into the JSON object
        let endpoints_value: Value = self.endpoints.iter().map(|e| e.to_value()).collect();
        node_obj.as_object_mut().unwrap().insert("end_points".to_string(), endpoints_value);

        node_obj
    }
}