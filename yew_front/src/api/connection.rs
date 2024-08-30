use gloo_net::Error;
use gloo_net::http::Request;
use serde_json::{Value, from_str, json};

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
    let response = Request::get(&format!("http://localhost:8080/get_json/{}", &ip))
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
    port: i32,
    user: String,
    password: String,
) -> Result<Value, String> {
    if let Ok(builder) = Request::post("http://localhost:8080/add_host")
        .header("Accept", "application/json")
        .json(&json!({
            "host": host,
            "port": port,
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
    let response = Request::get("http://localhost:8080/get_hosts")
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
    let response = Request::delete(&format!("http://localhost:8080/delete_host/{}", ip))
        .send()
        .await?;
    let text = response.text().await?;
    let json: Value = from_str(&text)?;
    Ok(json)
}
