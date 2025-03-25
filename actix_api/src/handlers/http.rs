use crate::AppError;
use crate::utils::{xml_to_json, find_token_key};
use reqwest::{Client, RequestBuilder};

use serde_json::Value;

#[derive(Debug)]
pub enum ContentType {
    Json,
    Xml,
    Unsupported,
}

impl ContentType {
    /// Determina el tipo de contenido basado en el Content-Type del header
    pub fn from_header(content_type: &str) -> Self {
        if content_type.contains("application/json") {
            ContentType::Json
        } else if content_type.contains("application/xml") || content_type.contains("text/xml") {
            ContentType::Xml
        } else {
            ContentType::Unsupported
        }
    }
}

pub struct HttpHandler;

impl HttpHandler {
    /// Builds a `RequestBuilder` for a GET request with specific configurations.
    ///
    /// # Arguments
    /// - `url`: A reference to the URL for the GET request.
    ///
    /// # Returns
    /// A `RequestBuilder` configured with the specified URL and headers.
    ///
    /// # Notes
    /// - The client accepts invalid SSL certificates.
    /// - Gzip, Brotli, and Deflate encodings are enabled for the request.
    /// - Adds headers for `Accept` and `Accept-Encoding` to handle JSON data and compression.
    fn client_get_builder(url: &String) -> RequestBuilder {
        Client::builder()
            .danger_accept_invalid_certs(true) // Accept invalid certificates.
            .gzip(true) // Enable gzip encoding.
            .brotli(true) // Enable brotli encoding.
            .deflate(true) // Enable deflate encoding.
            .build()
            .unwrap() // Handle client build error.
            .get(url) // Set up the GET request.
            .header("Accept", "*/*")
            .header("Accept-Encoding", "gzip, deflate, br")
    }

    async fn handle_content_type(response: reqwest::Response) -> Result<Value, AppError> {
        let headers = response.headers().clone();
            
        // Obtener el tipo de contenido del header
        let content_type_header = headers
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("");
        
        let content_type = ContentType::from_header(content_type_header);
        
        match content_type {
            ContentType::Json => {
                response
                .json::<Value>()
                .await
                .map_err(|err| AppError::validation_error(err.to_string()))
            }
            ContentType::Xml => {
                xml_to_json(
                    &response.text().await
                    .map_err(|err| AppError::RequestError(err.to_string()))?
                )
            },
            ContentType::Unsupported => Err(AppError::validation_error(content_type_header.to_string())),
        }
    }

    /// Sends a POST request with JSON data and returns the response as a `Value`.
    ///
    /// # Arguments
    /// - `url`: A reference to the URL for the POST request.
    /// - `json`: A reference to a `HashMap` containing the JSON payload for the POST request.
    ///
    /// # Returns
    /// A `Result` containing the deserialized JSON response as a `Value`, or an `Error`.
    ///
    /// # Notes
    /// - The client accepts invalid SSL certificates.
    /// - If the request or response parsing fails, appropriate errors are returned.
    pub async fn custom_post_request(
        url: &String,
        json: &Value,
    ) -> Result<Value, AppError> {
        let response = Client::builder()
            .danger_accept_invalid_certs(true) // Accept invalid certificates.
            .build()
            .unwrap() // Handle client build error.
            .post(url) // Set up the GET request.
            .json(&json)
            .send()
            .await
            .map_err(|err| AppError::request_error(err.to_string()))?;

        Ok(Self::handle_content_type(response).await?)
    }

    /// Sends a PUT request with JSON data and returns the response as a `Value`.
    ///
    /// # Arguments
    /// - `url`: A reference to the URL for the POST request.
    /// - `json`: A reference to a `HashMap` containing the JSON payload for the POST request.
    ///
    /// # Returns
    /// A `Result` containing the deserialized JSON response as a `Value`, or an `Error`.
    ///
    /// # Notes
    /// - The client accepts invalid SSL certificates.
    /// - If the request or response parsing fails, appropriate errors are returned.
    pub async fn custom_put_request(
        url: &String,
        json: &Value,
    ) -> Result<Value, AppError> {
        let response = Client::builder()
            .danger_accept_invalid_certs(true) // Accept invalid certificates.
            .build()
            .unwrap() // Handle client build error.
            .put(url) // Set up the GET request.
            .json(&json)
            .send()
            .await
            .map_err(|err| AppError::request_error(err.to_string()))?;

        Ok(Self::handle_content_type(response).await?)
    }

    /// Attempts to retrieve an token using a PUT/POST request.
    /// If the POST request fails or does not return a valid token, a PUT request is attempted.
    /// The function first looks for the token in the "token" field, and if not found, in the "accessSession" field.
    ///
    /// # Arguments
    /// * `url` - A reference to a string containing the request URL.
    /// * `json` - A reference to a hashmap containing the JSON request body.
    ///
    /// # Returns
    /// * `Ok(String)` - The token if found.
    /// * `Err(Error)` - An error if both requests fail or if the token cannot be found.
    pub async fn get_token(
        url: &String,
        json: &Value,
    ) -> Result<String, AppError> {
        let response = Self::custom_post_request(url, json).await;

        match response {
            Ok(res) => {
                match find_token_key(&res) {
                    Some(token_key) => {
                        return Ok(res
                            .get(token_key)
                            .ok_or_else(|| AppError::validation_error("Cannot find Token key in POST response"))?
                            .as_str()
                            .unwrap()
                            .to_string()
                        );
                    },
                    None => {
                        return Err(AppError::validation_error("Cannot find Token key in POST response"));
                    },
                }
                
            },
            Err(_) => {},
        }

        // If POST fails, try with PUT
        let response = Self::custom_put_request(url, json).await.map_err(|_| AppError::validation_error("Device has not response to POST/PUT request"))?;

        match find_token_key(&response) {
            Some(token_key) => {
                Ok(
                    response
                        .get(token_key)
                        .ok_or_else(|| AppError::validation_error("Cannot find Token key in PUT response"))?
                        .as_str()
                        .unwrap()
                        .to_string()
                )

            },
            None => Err(AppError::validation_error("Cannot find Token key in PUT response")),
        }
    }

    /// Sends a GET request with basic authentication and returns the response as a `Value`.
    ///
    /// # Arguments
    /// - `url`: A reference to the URL for the GET request.
    /// - `username`: The username for basic authentication.
    /// - `password`: An optional password for basic authentication.
    ///
    /// # Returns
    /// A `Result` containing the deserialized JSON response as a `Value`, or an `Error`.
    ///
    /// # Errors
    /// - Returns an error if the request fails or the response cannot be parsed as JSON.
    pub async fn basic_request(
        url: &String,
        username: String,
        password: Option<String>,
    ) -> Result<Value, AppError> {
        let response = Self::client_get_builder(url)
            .basic_auth(username, password)
            .send()
            .await
            .map_err(|err| AppError::request_error(err.to_string()))?;
    
        Ok(Self::handle_content_type(response).await?)
    }

    /// Sends a GET request with bearer token authentication and returns the response as a `Value`.
    ///
    /// # Arguments
    /// - `url`: A reference to the URL for the GET request.
    /// - `token`: The bearer token for authentication.
    ///
    /// # Returns
    /// A `Result` containing the deserialized JSON response as a `Value`, or an `Error`.
    ///
    /// # Errors
    /// - Returns an error if the request fails or the response cannot be parsed as JSON.
    pub async fn token_request(url: &String, token: &str) -> Result<Value, AppError> {
        Self::client_get_builder(url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|err| AppError::request_error(err.to_string()))?
            .json::<Value>()
            .await
            .map_err(|err| AppError::validation_error(err.to_string()))
    }
}