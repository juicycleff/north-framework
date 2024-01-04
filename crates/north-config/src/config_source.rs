use crate::serde_utils::Merge;
use convert_case::Casing;
use json_dotpath::DotPaths;
use serde::de;
use serde_json::Value;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;

#[cfg(any(feature = "tokio", feature = "async-std"))]
use async_trait::async_trait;
#[cfg(not(any(feature = "tokio", feature = "async-std")))]
use std::io::Read;

use crate::error::Error;
use crate::utils::{import_env_vars, preamble};
pub use convert_case::Case;

/// Trait for cloning a boxed `CustomConfigSource` object.
/// This is used in cases where we need to clone a trait object without knowing its concrete type.
pub trait CustomConfigSourceClone {
    fn clone_box(&self) -> Box<dyn CustomConfigSource>;
}

impl<T> CustomConfigSourceClone for T
where
    T: 'static + CustomConfigSource + Clone + Debug,
{
    fn clone_box(&self) -> Box<dyn CustomConfigSource> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CustomConfigSource> {
    fn clone(&self) -> Box<dyn CustomConfigSource> {
        self.clone_box()
    }
}

impl Debug for Box<dyn CustomConfigSource> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

/// Allows for providing a custom configuration source.
///
/// This trait can be implemented to define a custom configuration source. The trait
/// extends the `CustomConfigSourceClone` trait and requires the `Self` type to be
/// cloneable and support sending between threads (`Send`) and sharing between threads
/// (`Sync`).
#[cfg(not(any(feature = "tokio", feature = "async-std")))]
pub trait CustomConfigSource: CustomConfigSourceClone + Send + Sync {
    /// This is the only implementable member.
    /// Here you can return a serde value
    fn get_config_value(&self) -> Result<Value, Error>;
}

/// Allows you to provide a custom config source that can be accessed asynchronously.
///
/// This trait can be implemented to define a custom config source. The custom config source should implement the `CustomConfigSourceClone`, `Send`, and
#[cfg(any(feature = "tokio", feature = "async-std"))]
#[async_trait]
pub trait CustomConfigSource: CustomConfigSourceClone + Send + Sync {
    /// This is the only implementable member.
    /// Here you can return a serde value
    async fn get_config_value(&self) -> Result<Value, Error>;
}

/// Represents the source of configuration values.
///
/// This enum has three variants:
/// - `Env`: Represents loading configuration values from OS environment variables.
/// - `File`: Represents loading configuration values from a JSON, YAML, or TOML file.
/// - `Custom`: Represents loading configuration values using a custom implementation of `CustomConfigSource`.
///
/// # Examples
///
/// ## Using `Env` variant with default path
///
/// ```
/// use north_config::{ConfigSource, EnvSourceOptions};
///
/// let source = ConfigSource::Env(EnvSourceOptions::default());
/// ```
///
/// ## Using `Env` variant with custom path
///
/// ```
/// use north_config::{ConfigSource, EnvSourceOptions};
///
/// let source = ConfigSource::Env(EnvSourceOptions::default());
/// ```
///
/// ## Using `File` variant
///
/// ```
/// use north_config::ConfigSource;
///
/// let source = ConfigSource::File(String::from("config.json"), None);
/// ```
///
/// ## Using `Custom` variant with a custom implementation
///
/// ```
/// use north_config::{ConfigSource, CustomConfigSource, Error};
/// #[derive(Clone, Debug)]
/// struct MyCustomSource;
///
/// impl CustomConfigSource for MyCustomSource {
///     // implementation details here
///    fn get_config_value(&self) -> Result<serde_json::Value, Error> {
///        todo!()
///    }
/// }
///
/// let source = ConfigSource::Custom(Box::new(MyCustomSource));
/// ```
#[derive(Debug, Clone)]
pub enum ConfigSource {
    /// # Env
    /// loads from the OS env variables
    /// <br />
    ///
    ///
    /// # Example
    ///
    /// With default path
    /// ```rust
    ///
    /// use north_config::{ConfigSource, EnvSourceOptions};
    /// ConfigSource::Env(EnvSourceOptions::default());
    /// ```
    /// With custom path
    /// ```rust
    ///
    /// use north_config::{ConfigSource, EnvSourceOptions};
    /// ConfigSource::Env(EnvSourceOptions::default());
    /// ```
    Env(EnvSourceOptions),

    /// loads a json, YAML, OR TOML file
    File(String, Option<FileSourceOptions>),

    /// loads a json, YAML, OR TOML file
    Custom(Box<dyn CustomConfigSource>),
}

impl Default for ConfigSource {
    fn default() -> Self {
        ConfigSource::Env(EnvSourceOptions::default())
    }
}

/// # NorthConfigOptions
///
/// Represents the available options for initializing a `NorthConfig`.
///
/// ## Fields
///
/// - `sources`: A list of available configuration sources from the environment.
///   - Type: `Vec<ConfigSource>`
///   - Description: The potential sources of configuration data from the environment.
///   - Default: An empty vector (`Vec::new()`)
#[derive(Debug, Clone, Default)]
pub struct NorthConfigOptions {
    /// a list of available env sources
    pub sources: Vec<ConfigSource>,
}

impl NorthConfigOptions {
    pub fn new(sources: Vec<ConfigSource>) -> NorthConfigOptions {
        NorthConfigOptions { sources }
    }
}

/// # EnvSourceOptions
///
/// struct exposes available options for configuring Environmental source.
///
/// ## Fields
///
/// ### prefix
///
/// Environmental variable key prefix.
///
/// This field defaults to `Some("NORTH_".to_string())`.
///
/// ### nested_separator
///
/// Nested key separator.
///
/// This field defaults to `Some("__".to_string())`.
///
/// ### key_case
///
/// String case to deserialize key to. This must match your struct fields.
///
/// This field defaults to `Some(Case::Snake)`.
///
/// ### env_file_path
///
/// Accepts custom env file path to load up.
///
/// This field defaults to `Some("None".to_string())`.
///
/// ### watch
///
/// Enable datasource change watch (Only supports Env and File sources).
///
/// This field defaults to `false`.
#[derive(Debug, Clone)]
pub struct EnvSourceOptions {
    /// Environmental variable key prefix
    ///
    /// @defaults to ["NORTH_"]
    pub prefix: Option<String>,

    /// Nested key separator
    ///
    /// @defaults to ["__"]
    pub nested_separator: Option<String>,

    /// String case to deserialize key to. This must match your struct fields.
    ///
    /// @defaults to [Case::Snake]
    pub key_case: Option<Case>,

    /// Accepts custom env file path to load up
    ///
    /// @defaults to [None]
    pub env_file_path: Option<String>,

    /// Enable datasource change watch (Only supports Env and File sources)
    ///
    /// @defaults to False
    pub watch: bool,
}

impl Default for EnvSourceOptions {
    fn default() -> Self {
        EnvSourceOptions {
            prefix: Some("NORTH_".to_string()),
            nested_separator: Some("__".to_string()),
            key_case: Some(Case::Snake),
            env_file_path: None,
            watch: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileSourceOptions {
    pub skip_on_error: bool,
    pub enabled_environment: bool,
    pub watch: bool,
}

impl Default for FileSourceOptions {
    fn default() -> Self {
        FileSourceOptions {
            enabled_environment: true,
            skip_on_error: false,
            watch: false,
        }
    }
}

/// # NorthConfig
///
/// Struct representing a configuration value of type `T`.
///
/// This `struct` is used as a wrapper around the configuration value to provide additional functionality and
/// make it easier to work with.
///
/// ## Type parameters
/// - `T`: Represents the type of the configuration value, which must implement the `Clone` and `DeserializeOwned` traits.
///
/// ## Fields
/// - `value`: The actual configuration value of type `T`. It can be accessed and modified directly.
#[derive(Debug, Clone, Default)]
pub struct NorthConfig<T>
where
    T: Clone + de::DeserializeOwned,
{
    pub value: T,
}

impl<T: Clone + de::DeserializeOwned> NorthConfig<T> {
    /// access the configuration
    pub fn get_value(&self) -> &T {
        &self.value
    }
}

/// # new_config
///
/// creates a new instance of North Config. It accepts an array of data sources
///
/// Example
/// ```rust,ignore
///
/// #[derive(Clone, serde::Deserialize, Debug)]
/// struct DemoConfig {
///     pub host: Option<String>,
/// }
///  use north_config::{ConfigSource, EnvSourceOptions, NorthConfigOptions};
///  let config_options = NorthConfigOptions {
///     sources: vec![
///         // ConfigSource::File("/examples/configs/bootstrap.{{env}}.yaml".to_string()),
///         ConfigSource::Env(EnvSourceOptions::default()),
///     ],
///  };
///  let config = north_config::new_config::<DemoConfig>(config_options).await;
///  let config_val = config.get_value();
/// ```
#[cfg(any(feature = "tokio", feature = "async-std"))]
pub async fn new_config<T: Clone + de::DeserializeOwned>(
    option: NorthConfigOptions,
) -> NorthConfig<T> {
    preamble();

    let value = resolve_source::<T>(option).await;
    NorthConfig { value }
}

/// Creates a new `NorthConfig` with the specified options.
///
/// # Arguments
///
/// * `option` - The options to configure the `NorthConfig`.
///
/// # Returns
///
/// A new `NorthConfig` with the specified options.
///
/// # Example
///
/// ```rust
/// use north_config::{NorthConfigOptions, NorthConfig, EnvSourceOptions, ConfigSource};
///
/// envmnt::set("NORTH_NAMES__0__FIRST", "Run");
///
/// let mut env_opts = EnvSourceOptions::default();
/// env_opts.prefix = Some("NORTH".to_string());
///#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
///struct Names {
///    pub first: String
///}
/// #[derive(Debug, serde::Deserialize, Clone)]
/// struct MyConfig {
///    pub names: Vec<Names>
/// }
///
/// let option = NorthConfigOptions::new(vec![ConfigSource::Env(env_opts)]);  // replace with actual options
/// let config: NorthConfig<MyConfig> = crate::north_config::new_config(option);
/// ```
///
/// # Panics
///
/// This function may panic if the source for deserialization is missing or if deserialization fails.
#[cfg(not(any(feature = "tokio", feature = "async-std")))]
pub fn new_config<T: Clone + de::DeserializeOwned>(option: NorthConfigOptions) -> NorthConfig<T> {
    preamble();

    let value = resolve_source::<T>(option);
    NorthConfig { value }
}

/// Asynchronously resolves the configuration source for the given options.
///
/// # Arguments
///
/// * `option` - The NorthConfigOptions containing the configuration sources.
///
/// # Constraints
///
/// The type `T` must implement Clone and serde's DeserializeOwned trait.
///
/// # Returns
///
/// The deserialized configuration value of type `T`.
#[cfg(any(feature = "tokio", feature = "async-std"))]
async fn resolve_source<T>(option: NorthConfigOptions) -> T
where
    T: Clone + de::DeserializeOwned,
{
    let mut current_value = Value::default();
    let cargo_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or("./".to_string());

    #[cfg(debug_assertions)]
    let is_release = false;

    #[cfg(not(debug_assertions))]
    let is_release = true;

    for s in option.clone().sources {
        match s {
            ConfigSource::Env(env_opt) => {
                let value = resolve_env_source(env_opt);
                if value.is_some() {
                    current_value.merge(value.unwrap());
                }
            }
            ConfigSource::File(original_path, options) => {
                let value = resolve_file_source(
                    cargo_path.clone(),
                    original_path,
                    is_release,
                    options.unwrap_or_default(),
                )
                .await;
                if value.is_some() {
                    current_value.merge(value.unwrap());
                }
            }
            ConfigSource::Custom(source) => {
                let rsp = source.get_config_value().await;
                match rsp {
                    Ok(value) => {
                        if value.is_object() {
                            current_value.merge(value);
                        }
                    }
                    Err(_) => {
                        println!("Custom config was not loaded")
                    }
                }
            }
        };
    }

    serde_json::from_value::<T>(current_value).unwrap()
}

/// Resolves the configuration source based on the given options.
///
/// # Arguments
///
/// * `option` - The configuration options.
///
/// # Type Constraints
///
/// T must implement the `Clone` and `DeserializeOwned` traits.
///
/// # Returns
///
/// The resolved configuration source.
///
/// # Panics
///
/// This function will panic if the JSON value cannot be deserialized into the specified type.
#[cfg(not(any(feature = "tokio", feature = "async-std")))]
fn resolve_source<T>(option: NorthConfigOptions) -> T
where
    T: Clone + de::DeserializeOwned,
{
    let mut current_value = Value::default();
    let cargo_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "./".to_string());

    #[cfg(debug_assertions)]
    let is_release = false;

    #[cfg(not(debug_assertions))]
    let is_release = true;

    for s in option.sources {
        match s {
            ConfigSource::Env(env_opt) => {
                let value = resolve_env_source(env_opt);
                if let Some(v) = value {
                    current_value.merge(v);
                }
            }
            ConfigSource::File(original_path, options) => {
                let value = resolve_file_source(
                    cargo_path.clone(),
                    original_path,
                    is_release,
                    options.unwrap_or_default(),
                );
                if let Some(v) = value {
                    current_value.merge(v);
                }
            }
            ConfigSource::Custom(source) => {
                let rsp = source.get_config_value();
                match rsp {
                    Ok(value) => {
                        if value.is_object() {
                            current_value.merge(value);
                        }
                    }
                    Err(_) => {
                        println!("Custom config was not loaded")
                    }
                }
            }
        };
    }

    serde_json::from_value::<T>(current_value).unwrap()
}

/// Resolve env variable source to Serde [Value]
fn resolve_env_source(env_opt: EnvSourceOptions) -> Option<Value> {
    let env_options = env_opt.clone();

    // We want to load env if a path is passed
    if env_opt.env_file_path.is_some() {
        import_env_vars(env_opt.env_file_path.unwrap().as_str())
    }

    match process_envs(env_options) {
        Ok(value) => {
            if !value.is_null() {
                Some(value)
            } else {
                log::error!("Error loading env variables as config");
                None
            }
        }
        Err(error) => {
            println!("{:#?}", error);
            None
        }
    }
}

/// Resolves the source file specified by `original_path` by replacing the `{{env}}` placeholder
/// with either "release" or "debug" depending on the value of `is_release`.
///
/// The resolved file path is obtained by joining `original_path` with `cargo_path`.
/// If the resolved file path does not exist, a panic with an appropriate message is raised.
///
/// The resolved file path is then passed to the `read_file_value` function to read the contents of the file asynchronously.
///
/// If the value read from the file is not null, it is returned wrapped in an `Option`.
/// Otherwise, an error message is logged and `None` is returned.
///
/// # Arguments
///
/// * `cargo_path` - The path to the cargo project.
/// * `original_path` - The original source file path with the `{{env}}` placeholder.
/// * `is_release` - A flag indicating whether the build is a release build or not.
///
/// # Returns
///
/// An `Option` containing the value read from the resolved source file, if it exists.
/// Otherwise, `None` is returned.
#[cfg(any(feature = "tokio", feature = "async-std"))]
async fn resolve_file_source(
    cargo_path: String,
    original_path: String,
    is_release: bool,
    options: FileSourceOptions,
) -> Option<Value> {
    let path = if options.enabled_environment {
        match is_release {
            true => original_path.replace("{{env}}", "release"),
            false => original_path.replace("{{env}}", "debug"),
        }
    } else {
        original_path.clone()
    };

    let path_buf = PathBuf::from(cargo_path.clone()).join(path.clone());
    if !path_buf.exists() && !options.skip_on_error {
        panic!("No file found in path: {}", path.clone());
    }

    if !path_buf.exists() && options.skip_on_error {
        return None;
    }

    let file_path = path_buf.display().to_string();
    let value = read_file_value(file_path).await;

    if !value.is_null() {
        Some(value)
    } else {
        log::error!("Error loading config file {original_path}");
        None
    }
}

/// Resolves a file source based on the given parameters.
///
/// # Arguments
///
/// - `cargo_path`: The path to the cargo directory.
/// - `original_path`: The original path to the file source.
/// - `is_release`: A flag indicating whether it's a release build or not.
///
/// # Returns
///
/// - `Some(Value)`: The value read from the file source if it exists and is not null.
/// - `None`: If the file source does not exist or the value read is null.
///
/// # Panics
///
/// - If no file is found in the resolved path.
#[cfg(not(any(feature = "tokio", feature = "async-std")))]
fn resolve_file_source(
    cargo_path: String,
    original_path: String,
    is_release: bool,
    options: FileSourceOptions,
) -> Option<Value> {
    let path = if options.enabled_environment {
        match is_release {
            true => original_path.replace("{{env}}", "release"),
            false => original_path.replace("{{env}}", "debug"),
        }
    } else {
        original_path.clone()
    };

    let path_buf = PathBuf::from(cargo_path).join(path.clone());
    if !path_buf.exists() && !options.skip_on_error {
        panic!("No file found in path: {}", path);
    }
    if !path_buf.exists() && options.skip_on_error {
        return None;
    }

    let file_path = path_buf.display().to_string();
    let value = read_file_value(file_path);

    if !value.is_null() {
        Some(value)
    } else {
        log::error!("Error loading config file {original_path}");
        None
    }
}

/// Processes environment variables based on provided options.
///
/// # Arguments
///
/// * `option` - The `EnvSourceOptions` containing the processing options.
///
/// # Returns
///
/// Returns a `Result` containing the processed environment variables as a `Value` or an `Error` if an error occurs.
///
/// # Example
///
/// ```
/// use serde_json::Value;
/// use std::error::Error;
///
/// #[derive(Default)]
/// struct EnvSourceOptions {
///     prefix: Option<String>,
///     nested_separator: Option<String>,
///     key_case: Option<Case>,
/// }
///
/// #[derive(Debug)]
/// enum Case {
///     Snake,
///     Camel,
///     Pascal,
///     Kebab,
/// }
///
/// fn process_envs(option: EnvSourceOptions) -> Result<Value, dyn Error> {
///     // Implementation goes here
///     Ok(Value::default())
/// }
/// ```
fn process_envs(option: EnvSourceOptions) -> Result<Value, Error> {
    let temp_prefix = option.prefix.unwrap_or_else(|| "NORTH_".to_string());
    let prefix: &str = temp_prefix.as_str();

    let nested_separator = option.nested_separator.unwrap_or_else(|| "__".to_string());
    let separator: &str = nested_separator.as_str();

    let case: Case = option.key_case.unwrap_or(Case::Snake);
    let mut obj = Value::Null;

    for (key, value) in std::env::vars() {
        if !key.starts_with(prefix) {
            continue;
        }

        let new_key = key.strip_prefix(prefix).expect("env var prefix missing");
        let dot_key: String = new_key.replace(separator.clone(), ".").clone().to_case(case);
        let new_value: serde_json::Value = serde_json::from_str(value.as_str()).expect("go value");
        obj.dot_set(dot_key.as_str(), new_value).unwrap();
    }

    Ok(obj)
}

/// Asynchronously reads the contents of a file at the specified path and converts it to a `serde_json::Value`.
/// Supports either the `tokio` or `async-std` runtime, depending on the enabled feature flag.
///
/// # Arguments
///
/// * `path` - A `String` representing the file path to read.
///
/// # Returns
///
/// An asynchronous task that resolves to a `serde_json::Value` representing the contents of the file.
///
/// # Panics
///
/// This function will panic if it is unable to open the file or if there is an error while reading from it.
#[cfg(any(feature = "tokio", feature = "async-std"))]
async fn read_file_value(path: String) -> Value {
    let mut contents = String::new();

    #[cfg(feature = "tokio")]
    {
        use tokio::io::AsyncReadExt;
        let mut file = tokio::fs::File::open(path.clone())
            .await
            .expect("Unable to open file");
        file.read_to_string(&mut contents).await.unwrap();
    }

    #[cfg(feature = "async-std")]
    {
        use async_std::io::ReadExt;
        let mut file = async_std::fs::File::open(path.clone())
            .await
            .expect("Unable to open file");
        file.read_to_string(&mut contents).await.unwrap();
    };

    convert_str_to_value(path, contents)
}

fn convert_str_to_value(path: String, contents: String) -> Value {
    if path.ends_with(".yaml") || path.ends_with(".yml") {
        #[cfg(not(feature = "yaml"))]
        {
            panic!("missing yaml feature for crate, please enable yaml feature")
        }

        #[cfg(feature = "yaml")]
        {
            let yaml: Value = serde_yaml::from_str::<Value>(&contents)
                .expect("YAML does not have correct format.");
            yaml
        }
    } else if path.ends_with(".toml") {
        #[cfg(not(feature = "toml"))]
        {
            panic!("missing toml feature for crate, please enable toml feature")
        }

        #[cfg(feature = "toml")]
        {
            let rsp: Value =
                toml::from_str::<Value>(&contents).expect("TOML does not have correct format.");
            rsp
        }
    } else if path.ends_with(".json") {
        let json: Value =
            serde_json::from_str(&contents).expect("JSON does not have correct format.");
        json
    } else if path.ends_with(".ron") {
        #[cfg(not(feature = "ron"))]
        {
            panic!("missing ron feature for crate, please enable ron feature")
        }

        #[cfg(feature = "ron")]
        {
            let data = ron::de::from_str(&contents).expect("RON does not have correct format.");
            dbg!(contents.clone());
            data
        }
    } else {
        let json: Value =
            serde_json::from_str(&contents).expect("JSON does not have correct format.");
        json
    }
}

/// Reads the contents of a file and converts it to a `Value`.
///
/// # Arguments
///
/// * `path` - A `String` that represents the path of the file to be read.
///
/// # Panics
///
/// This function will panic if the file cannot be opened.
///
/// # Returns
///
/// A `Value` representing the contents of the file.
#[cfg(not(any(feature = "tokio", feature = "async-std")))]
fn read_file_value(path: String) -> Value {
    let mut contents = String::new();
    let mut file = std::fs::File::open(path.clone()).expect("Unable to open file");
    file.read_to_string(&mut contents).unwrap();

    convert_str_to_value(path, contents)
}
