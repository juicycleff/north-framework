use std::fmt::Display;

/// # Error
///
/// Custom error enum for handling various kinds of errors in the application.
///
/// The `Error` enum has several variants:
///
/// - `IoError`: Represents an I/O error encountered while performing file I/O operations. It contains an `std::io::Error` as the inner error.
/// - `JsonParseError`: Represents an error encountered while parsing JSON data. It contains a `serde_json::Error` as the inner error.
/// - `RonParseError`: Represents an error encountered while parsing RON data. This variant is only available if the `"ron"` feature is enabled. It contains
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    JsonParseError(serde_json::Error),
    #[cfg(feature = "ron")]
    RonParseError(ron::Error),
    #[cfg(feature = "yaml")]
    YamlParseError(serde_yaml::Error),
    #[cfg(feature = "toml")]
    TomlParseError(toml::de::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(io_error) => write!(f, "{}", io_error),
            Error::JsonParseError(io_error) => write!(f, "{}", io_error),
            #[cfg(feature = "ron")]
            Error::RonParseError(error) => write!(f, "{}", error),
            #[cfg(feature = "yaml")]
            Error::YamlParseError(error) => write!(f, "{}", error),
            #[cfg(feature = "toml")]
            Error::TomlParseError(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonParseError(e)
    }
}

#[cfg(feature = "ron")]
impl From<ron::Error> for Error {
    fn from(e: ron::Error) -> Self {
        Error::RonParseError(e)
    }
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::YamlParseError(e)
    }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::TomlParseError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}
