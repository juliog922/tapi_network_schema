use serde::{Deserialize, Serialize};
use serde_json::Value;

// ==== Core ====

/// Represents a network device with its IP address, optional port, and authentication details.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Device {
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(skip_serializing)]
    pub auth: Auth, // Authentication method (enum)
}

/// Enum representing the different authentication methods
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Auth {
    Basic(BasicAuth),
    Token(TokenAuth),
}

/// Represents Basic Authentication with username and password
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

/// Represents Token Authentication with an arbitrary body and authentication URI
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TokenAuth {
    pub auth_body: Value, // A JSON object containing Token authentication data
    pub auth_uri: String, // URI for Token authentication
}

// ==== Implementation ====

impl Device {
    pub fn get_device_base_url(&self) -> String {
        format!(
            "http://{}{}",
            &self.ip,
            &self.port.map(|p| format!(":{}", p)).unwrap_or_default()
        )
    }

    pub fn get_full_auth_url(&self) -> String {
        match &self.auth {
            Auth::Basic(_) => self.get_device_base_url(),
            Auth::Token(token_auth) => {
                format!(
                    "{}/{}",
                    self.get_device_base_url(),
                    &token_auth
                        .auth_uri
                        .strip_prefix("/")
                        .unwrap_or(&token_auth.auth_uri)
                )
            }
        }
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.auth {
            Auth::Basic(basic_auth) => {
                write!(
                    f,
                    r#"
                    ip: {},
                    port: {:?},
                    auth_type: basic,
                    username: {},
                    password: {}, 
                "#,
                    self.ip, self.port, basic_auth.username, basic_auth.password
                )
            }
            Auth::Token(token_auth) => {
                write!(
                    f,
                    r#"
                    ip: {},
                    port: {:?},
                    auth_type: token,
                    auth_body: {:?}, 
                    auth_uri: {:?},
                "#,
                    self.ip, self.port, token_auth.auth_body, token_auth.auth_uri
                )
            }
        }
    }
}
