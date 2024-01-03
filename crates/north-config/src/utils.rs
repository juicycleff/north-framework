use std::fs;

/// Set up the environment and logging configuration for the application.
///
/// This function performs the following tasks:
/// - Imports environment variables from the ".env" file in the root directory.
/// - Sets the default value for the "RUST_LOG" environment variable if it is not already set.
/// - Initializes the logging configuration for the application.
///
/// # Examples
///
/// ```rust
/// preamble();
/// ```
pub(crate) fn preamble() {
    import_env_vars("/.env");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    // enable tracing
    // tracing_subscriber::fmt::init();
}

/// Imports environment variables from a file.
///
/// # Arguments
///
/// * `file_path` - A string slice containing the path to the file to import.
///
/// # Example
///
/// ```rust
/// use crate::my_module::import_env_vars;
///
/// import_env_vars("config.env");
/// ```
///
/// # Panics
///
/// This function panics if the specified file does not exist or if there is an error while
/// loading the environment variables from the file.
pub(crate) fn import_env_vars(file_path: &str) {
    let path_buf = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let path = format!("{}/{}", path_buf, file_path);

    if path_exists(path.as_str()) {
        dotenvy::from_path(path).unwrap();
    }
}

/// Checks if a file or directory exists at the given path.
///
/// # Arguments
///
/// * `path` - The path to the file or directory.
///
/// # Returns
///
/// Returns `true` if a file or directory exists at the given path, and `false` otherwise.
///
/// # Examples
///
/// ```rust
/// let exists = path_exists("/path/to/file.txt");
/// if exists {
///     println!("File exists!");
/// } else {
///     println!("File does not exist.");
/// }
/// ```
pub(crate) fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
