use std::fs;

/// Some North config preamble that setup env var from file
pub(crate) fn preamble() {
    import_env_vars("/.env");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    // enable tracing
    // tracing_subscriber::fmt::init();
}

/// Some North config preamble that setup env var from file
pub(crate) fn import_env_vars(file_path: &str) {
    let path_buf = std::env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string());
    let path = format!("{}/{}", path_buf, file_path);

    if path_exists(path.as_str()) {
        dotenvy::from_path(path).unwrap();
    }
}

/// checks if path exists in a dir
pub(crate) fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
