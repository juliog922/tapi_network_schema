use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Device {
    pub ip: String, // Host name or IP address of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>, // Optional port number
    pub auth: Auth, // Authentication method (enum)
}

/// Enum representing the different authentication methods
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Auth {
    Oauth2(Oauth2),       // OAuth2 Authentication
    BasicAuth(BasicAuth), // Basic Authentication
    Custom(CustomAuth),   // Custom Authentication
}

/// Represents Basic Authentication with username and password
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BasicAuth {
    pub username: String, // Username for authentication
    pub password: String, // Password for authentication
}

/// Represents OAuth2 Authentication with additional fields for grant type and authentication URL
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Oauth2 {
    pub username: String,   // Username for OAuth2 authentication
    pub password: String,   // Password for OAuth2 authentication
    pub grant_type: String, // Grant type for OAuth2 (e.g., client_credentials, password)
    pub auth_sufix: String, // URL to request OAuth2 token
}

/// Represents Custom Authentication with an arbitrary body and authentication URL
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CustomAuth {
    pub auth_body: Value,   // A JSON object containing custom authentication data
    pub auth_sufix: String, // URL for custom authentication
}
