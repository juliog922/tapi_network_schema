#[cfg(test)]
mod tests {
    // Import the necessary structs and enums from your backend models
    use actix_api::models::devices::{Auth, Device};

    // Import necessary modules from serde_json for JSON handling
    use serde_json::{from_str, Value};

    /// Test case for creating a `Device` with Basic Authentication
    #[test]
    fn test_basic_device() {
        // Example JSON data for Basic Authentication
        let json_data = r#"
        {
            "ip": "10.95.87.21",
            "port": 18010,
            "auth": {
                "username": "tapi",
                "password": "2025_T3st",
                "foo":"bar"
            },
            "foo":"bar"
        }
        "#;

        // Convert the JSON string into a serde_json::Value
        let json_value: Value =
            from_str(&json_data).expect("Json test data cannot be transformed to Value type");

        // Create a `Device` instance from the JSON `Value`
        let device: Device = serde_json::from_value(json_value).expect("Device cannot be created");

        // Match the type of authentication used in the device
        match device.auth {
            // Check if it uses Basic Authentication
            Auth::Basic(_) => assert!(true), // Pass if it's BasicAuth
            _ => panic!("There isn't Basic Authentication here"), // Fail if it's not BasicAuth
        }
    }

    /// Test case for creating a `Device` with Token Authentication
    #[test]
    fn test_token_device() {
        // Example JSON data for Token Authentication
        let json_data = r#"
        {
            "ip": "10.95.86.185",
            "auth": {
                "auth_body": {
                    "username": "admin",
                    "password": "Telef@12!",
                    "foo":"bar"
                },
                "auth_uri": "/tron/api/v1/tokens",
                "foo":"bar"
            }
        }
        "#;

        // Convert the JSON string into a serde_json::Value
        let json_value: Value =
            from_str(&json_data).expect("Json test data cannot be transformed to Value type");

        // Create a `Device` instance from the JSON `Value`
        let device: Device = serde_json::from_value(json_value).expect("Device cannot be created");

        // Match the type of authentication used in the device
        match device.auth {
            // Check if it uses Token Authentication
            Auth::Token(_) => assert!(true), // Pass if it's TokenAuth
            _ => panic!("There isn't Token Authentication here"), // Fail if it's not TokenAuth
        }
    }
}
