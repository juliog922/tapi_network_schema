use serde_json::{Value, Map};
use crate::models::node::Node;

/// Represents a service in the network schema.
///
/// A `Service` is an entity within the network schema that includes a unique identifier, an optional
/// name, and a list of associated nodes. This structure supports representing and serializing a service
/// in a JSON-compatible format.
#[derive(Debug, Clone)]
pub struct Service {
    /// Unique identifier for the service.
    pub uuid: Value,

    /// Optional name for the service.
    pub name: Option<Value>,

    /// List of nodes associated with this service.
    pub nodes: Vec<Node>,
}

impl Service {
    /// Constructs a new `Service` with the specified UUID and optional name.
    ///
    /// # Parameters
    /// - `uuid`: The unique identifier for the service.
    /// - `name`: An optional name for the service.
    ///
    /// # Returns
    /// A new instance of `Service` with the provided UUID and name, and an empty list of nodes.
    pub fn new(uuid: Value, name: Option<Value>) -> Self {
        Service {
            uuid,
            name,
            nodes: vec![],
        }
    }

    /// Adds a node to the list of nodes associated with this service.
    ///
    /// # Parameters
    /// - `node`: The `Node` to be added to the service.
    ///
    /// # Returns
    /// This method does not return any value. It modifies the internal state of the `Service` by adding
    /// the provided node to its list of nodes.
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    /// Converts the `Service` instance into a `serde_json::Value`.
    ///
    /// This method creates a JSON object representing the service, including its UUID, optional name,
    /// and a list of associated nodes. Each node is converted to a JSON value using its `to_value` method.
    ///
    /// # Returns
    /// A `serde_json::Value` representing the `Service` as a JSON object. The object includes:
    /// - `uuid`: The UUID of the service.
    /// - `value_name`: The optional name of the service (if present).
    /// - `nodes`: A JSON array containing the serialized nodes.
    pub fn to_value(&self) -> Value {
        // Create a new JSON object to hold the service data
        let mut service_obj = Value::Object(Map::new());

        // Insert the service UUID into the JSON object
        service_obj.as_object_mut().unwrap().insert("uuid".to_string(), self.uuid.clone());

        // Conditionally insert the optional name into the JSON object if it is present
        if let Some(ref name) = self.name {
            service_obj.as_object_mut().unwrap().insert("value_name".to_string(), name.clone());
        }

        // Convert the list of nodes into a JSON array and insert it into the JSON object
        let nodes_value: Value = self.nodes.iter().map(|n| n.to_value()).collect();
        service_obj.as_object_mut().unwrap().insert("nodes".to_string(), nodes_value);

        service_obj
    }
}
