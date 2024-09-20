use gloo_net::Error;
use gloo_net::http::Request;
use serde_json::{Value, from_str, json};

const API_URL: &'static str = "/api";

/// Fetches JSON schema from the server for a given IP address.
///
/// # Arguments
///
/// * `ip` - The IP address for which the JSON schema is to be fetched.
///
/// # Returns
///
/// * `Result<Value, Error>` - Returns the JSON schema as a `Value` if successful, or an error if the request fails.
pub async fn get_schema(ip: String) -> Result<Value, Error> {
    let response = Request::get(&format!("{}/get_json/{}",API_URL, &ip))
        .send()
        .await?;
    let text = response.text().await?;
    let json: Value = from_str(&text)?;
    Ok(json)
}

/// Adds a new device to the server.
///
/// # Arguments
///
/// * `host` - The host address of the device.
/// * `port` - The port number of the device.
/// * `user` - The username for accessing the device.
/// * `password` - The password for accessing the device.
///
/// # Returns
///
/// * `Result<Value, String>` - Returns the server's response as a `Value` if successful, or an error message if the request fails.
pub async fn add_device(
    host: String,
    port: Option<String>,
    tenant: Option<String>,
    user: String,
    password: String,
) -> Result<Value, String> {
    let real_port: Option<String>;

    if port.is_some() {
        if port.clone().unwrap() == "".to_string() {
            real_port = None;
        } else {
            real_port = port.clone();
        }
    } else {
        real_port = None;
    }

    let real_tenant: Option<String>;

    if tenant.is_some() {
        if tenant.clone().unwrap() == "".to_string() {
            real_tenant = None;
        } else {
            real_tenant = tenant.clone();
        }
    } else {
        real_tenant = None;
    }

    if let Ok(builder) = Request::post(&format!("{}/add_host", API_URL))
        .header("Accept", "application/json")
        .json(&json!({
            "host": host,
            "port": real_port,
            "tenant": real_tenant,
            "user": user,
            "password": password,
        })) {
            if let Ok(response) = builder.send().await {
                if let Ok(text) = response.text().await {
                    if let Ok(json) = from_str(&text) {
                        Ok(json)
                    } else {
                        Err(String::from("Failed to parse response JSON."))
                    }
                } else {
                    Err(String::from("Empty API response. The API might not be available."))
                }
            } else {
                Err(String::from("Request failed. The API might not be available."))
            }
    } else {
        Err(String::from("Failed to create request. Cannot add new device."))
    }
}

/// Fetches the list of devices from the server.
///
/// # Returns
///
/// * `Result<Value, Error>` - Returns the list of devices as a `Value` if successful, or an error if the request fails.
pub async fn get_devices() -> Result<Value, Error> {
    let response = Request::get(&format!("{}/get_hosts", API_URL))
        .send()
        .await?;
    let text = response.text().await?;
    let json: Value = from_str(&text)?;
    Ok(json)
}

/// Deletes a device from the server.
///
/// # Arguments
///
/// * `ip` - The IP address of the device to be deleted.
///
/// # Returns
///
/// * `Result<Value, Error>` - Returns the server's response as a `Value` if successful, or an error if the request fails.
pub async fn delete_device(ip: &str) -> Result<Value, Error> {
    let response = Request::delete(&format!("{}/delete_host/{}",API_URL, ip))
        .send()
        .await?;
    let text = response.text().await?;
    let json: Value = from_str(&text)?;
    Ok(json)
}
