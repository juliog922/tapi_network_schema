use serde::Deserialize;

//pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
pub enum Error {
    #[allow(dead_code)]
    Custom(String),
}

impl Error {
    pub fn _custom(value: impl std::fmt::Display) -> Self {
        Self::Custom(value.to_string())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}
