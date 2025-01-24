use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a network device with its IP address, optional port, and authentication details.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Device {
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    pub auth: Auth, // Authentication method (enum)
}

/// Enum representing the different authentication methods
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Auth {
    Oauth2(Oauth2),
    Basic(BasicAuth),
    Custom(CustomAuth),
}

/// Represents Basic Authentication with username and password
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

/// Represents OAuth2 Authentication with additional fields for grant type and authentication URL
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Oauth2 {
    pub username: String,
    pub password: String,
    pub grant_type: String, // Grant type for OAuth2 (e.g., client_credentials, password)
    pub auth_sufix: String,
}

/// Represents Custom Authentication with an arbitrary body and authentication URL
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CustomAuth {
    pub auth_body: Value,   // A JSON object containing custom authentication data
    pub auth_sufix: String, // URL for custom authentication
}
