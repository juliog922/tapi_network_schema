#[cfg(test)]
mod tests {
    // Import the necessary structs and enums from your backend models
    use actix_api::models::files_model::FilesEnum;
    // Import necessary modules from serde_json for JSON handling
    use serde_json::json;

    #[test]
    fn test_filesenum_complete() {
        let json_data = json!({
            "id": "test",
            "complete_context_path": "/path/to/context.json"
        });

        let result: FilesEnum =
            serde_json::from_value(json_data).expect("Failed to deserialize FilesEnum as Complete");

        match result {
            FilesEnum::Complete(complete) => {
                assert_eq!(complete.id, "test");
                assert_eq!(complete.complete_context_path, "/path/to/context.json");
            }
            _ => panic!("Expected FilesEnum::Complete"),
        }
    }

    #[test]
    fn test_filesenum_bypart() {
        let json_data = json!({
            "id": "test_1",
            "topology_path": "/path/to/topology.json",
            "connections_path": "/path/to/connections.json",
            "connectivity_services_path": "/path/to/services.json"
        });

        let result: FilesEnum =
            serde_json::from_value(json_data).expect("Failed to deserialize FilesEnum as ByPart");

        match result {
            FilesEnum::ByPart(bypart) => {
                assert_eq!(bypart.id, "test_1");
                assert_eq!(bypart.topology_path, "/path/to/topology.json");
                assert_eq!(bypart.connections_path, "/path/to/connections.json");
                assert_eq!(bypart.connectivity_services_path, "/path/to/services.json");
            }
            _ => panic!("Expected FilesEnum::ByPart"),
        }
    }

    #[test]
    fn test_filesenum_invalid() {
        let json_data = json!({
            "id": "test_2"
        });

        let result: Result<FilesEnum, _> = serde_json::from_value(json_data);

        assert!(result.is_err(), "Expected deserialization to fail");
    }
}
