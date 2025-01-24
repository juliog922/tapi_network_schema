use serde::Deserialize;

/// Represents possible errors in the application, including custom error messages.
#[derive(Debug, Deserialize)]
pub enum Error {
    #[allow(dead_code)]
    Custom(String),
}

impl Error {
    /// Creates a custom error from any type that implements `std::fmt::Display`.
    ///
    /// # Arguments
    /// - `value`: The value to be converted into a custom error message.
    ///
    /// # Returns
    /// - `Error::Custom`: A custom error instance with the given message.
    pub fn _custom(value: impl std::fmt::Display) -> Self {
        Self::Custom(value.to_string())
    }
}

impl From<&str> for Error {
    /// Converts a string slice (`&str`) into a custom error.
    ///
    /// # Arguments
    /// - `value`: The string slice to be converted into an error.
    ///
    /// # Returns
    /// - `Error::Custom`: A custom error instance with the given string message.
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}
