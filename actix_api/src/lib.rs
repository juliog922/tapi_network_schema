pub mod logic; // Expose to bin
pub mod models; // Expose to make tests
pub mod routes; // Expose to bin
pub mod utils; // Expose to make tests

use std::fmt;

/// Enum representing various application errors.
#[derive(Debug)]
pub enum AppError {
    /// Error in HTTP connection
    RequestError(String),

    /// Error while validating input data
    ValidationError(String),

    /// Error on Algorithms execution
    LogicError(String),

    /// Error in the schemas definition
    ModelError(String),

    ///Error on database interaction
    DatabaseError(String),
}

impl AppError {
    /// Constructor for RequestError.
    pub fn request_error(msg: impl Into<String>) -> Self {
        let message = msg.into();
        AppError::RequestError(message)
    }

    /// Constructor for ValidationError.
    pub fn validation_error(msg: impl Into<String>) -> Self {
        let message = msg.into();
        AppError::ValidationError(message)
    }

    /// Constructor for LogicError.
    pub fn logic_error(msg: impl Into<String>) -> Self {
        let message = msg.into();
        AppError::LogicError(message)
    }

    /// Constructor for ModelError.
    pub fn model_error(msg: impl Into<String>) -> Self {
        let message = msg.into();
        AppError::ModelError(message)
    }

    /// Constructor for DatabaseError.
    pub fn database_error(msg: impl Into<String>) -> Self {
        let message = msg.into();
        AppError::DatabaseError(message)
    }
}

impl fmt::Display for AppError {
    /// Formats the `AppError` enum into a human-readable string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::RequestError(msg) => write!(f, "{}", msg),
            AppError::ValidationError(msg) => write!(f, "{}", msg),
            AppError::LogicError(msg) => write!(f, "{}", msg),
            AppError::ModelError(msg) => write!(f, "{}", msg),
            AppError::DatabaseError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for AppError {}
