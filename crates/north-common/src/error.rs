use std::fmt::Display;

/// # Error
///
/// Custom error
#[derive(Debug)]
pub enum Error {
    InternalServerError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServerError(io_error) => write!(f, "{}", io_error),
        }
    }
}

impl std::error::Error for Error {}

/// Convert std::io::Error to NorthErrors
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::InternalServerError(error.to_string())
    }
}

/// Convert serde_yaml::Error to NorthErrors
impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Error {
        Error::InternalServerError(error.to_string())
    }
}
