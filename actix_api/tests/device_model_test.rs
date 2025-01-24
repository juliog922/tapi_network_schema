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
                "username": "123",
                "password": "Zte2024!"
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
            // Check if it uses Basic Authentication
            Auth::Basic(_) => assert!(true), // Pass if it's BasicAuth
            _ => panic!("There isn't Basic Authentication here"), // Fail if it's not BasicAuth
        }
    }

    /// Test case for creating a `Device` with Custom Authentication
    #[test]
    fn test_custom_device() {
        // Example JSON data for Custom Authentication
        let json_data = r#"
        {
            "ip": "10.95.86.185",
            "auth": {
                "auth_body": {
                    "username": "admin",
                    "password": "Telef@12!"
                },
                "auth_sufix": "/tron/api/v1/tokens"
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
            // Check if it uses Custom Authentication
            Auth::Custom(_) => assert!(true), // Pass if it's CustomAuth
            _ => panic!("There isn't Custom Authentication here"), // Fail if it's not CustomAuth
        }
    }

    /// Test case for creating a `Device` with OAuth2 Authentication
    #[test]
    fn test_oauth2_device() {
        // Example JSON data for OAuth2 Authentication
        let json_data = r#"
        {
            "ip": "10.95.87.21",
            "auth": {
                "username": "admin",
                "password": "Devops1.!",
                "grant_type": "client_credentials",
                "auth_sufix": "/rest-gateway/rest/api/v1/auth/token"
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
            // Check if it uses OAuth2 Authentication
            Auth::Oauth2(_) => assert!(true), // Pass if it's Oauth2Auth
            _ => panic!("There isn't OAuth2 Authentication here"), // Fail if it's not Oauth2Auth
        }
    }
}
