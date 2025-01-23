use serde_json::Value;

use crate::{models::connectivity_services::SimpleService, utils::find_name};

/// Builds a vector of `SimpleService` objects from a JSON array of connectivity services.
///
/// # Arguments
/// - `connectivity_service_json`: A reference to a vector of JSON values representing connectivity services.
///
/// # Returns
/// A vector of `SimpleService` objects containing the UUID and name of each service.
pub fn connectivity_services_vector_build(
    connectivity_service_json: &Vec<Value>,
) -> Vec<SimpleService> {
    let mut connectivity_services_vector: Vec<SimpleService> = Vec::new();

    for service in connectivity_service_json {
        connectivity_services_vector.push(SimpleService {
            uuid: service.get("uuid").unwrap_or(&Value::default()).to_string(),
            name: find_name(&service, "SERVICE_NAME".to_string()),
        });
    }

    connectivity_services_vector
}
