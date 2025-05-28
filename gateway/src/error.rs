use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use awc::error::SendRequestError;
use serde_yaml::Error as YamlError;
use std::io;
use thiserror::Error;

/// Global application error type for the API Gateway.
#[derive(Debug, Error)]
pub enum AppError {
    // ─────── Routing / Gateway Errors ───────
    /// The requested route was not found in the configuration.
    #[error("Route not found for path: {0}")]
    RouteNotFound(String),

    /// Invalid configuration format or structure.
    #[error("Configuration error: {0}")]
    InvalidConfig(String),

    /// Error forwarding request to upstream service.
    #[error("Upstream request failed: {0}")]
    Upstream(#[from] SendRequestError),

    // ─────── Parsing & System Errors ───────
    /// YAML parsing failure.
    #[error("YAML deserialization failed: {0}")]
    YamlParse(#[from] YamlError),

    /// File I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    // ─────── Catch-all ───────
    /// Any other unexpected error.
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

// Optional: make AppError usable directly in Actix responses
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::RouteNotFound(_) => StatusCode::NOT_FOUND,
            AppError::InvalidConfig(_) => StatusCode::BAD_REQUEST,
            AppError::Upstream(_) => StatusCode::BAD_GATEWAY,
            AppError::YamlParse(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
