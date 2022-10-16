use crate::serde_utils::Merge;
use convert_case::Casing;
use json_dotpath::DotPaths;
use serde::de;
use serde_json::Value;
use std::path::PathBuf;

#[cfg(not(any(feature = "tokio", feature = "async-std")))]
use std::io::Read;

use crate::error::Error;
use crate::utils::{import_env_vars, preamble};
pub use convert_case::Case;

/// # ConfigSource
///
/// Describes the various sources for config that is supported
/// Current supports file and env vars
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
    File(String),
}

impl Default for ConfigSource {
    fn default() -> Self {
        ConfigSource::Env(EnvSourceOptions::default())
    }
}

/// # NorthConfigOptions
///
/// struct exposes available options for initializing NorthConfig
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
/// Options used to deserialized env var to rust struct
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
}

impl Default for EnvSourceOptions {
    fn default() -> Self {
        EnvSourceOptions {
            prefix: Some("NORTH_".to_string()),
            nested_separator: Some("__".to_string()),
            key_case: Some(Case::Snake),
            env_file_path: None,
        }
    }
}

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

#[cfg(not(any(feature = "tokio", feature = "async-std")))]
/// # new_config
///
/// creates a new instance of North Config. It accepts an array of data sources
///
/// Example
/// ```rust,ignore
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
///  let config = north_config::new_config::<DemoConfig>(config_options);
///  let config_val = config.get_value();
/// ```
pub fn new_config<T: Clone + de::DeserializeOwned>(option: NorthConfigOptions) -> NorthConfig<T> {
    preamble();

    let value = resolve_source::<T>(option);
    NorthConfig { value }
}

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
            ConfigSource::File(original_path) => {
                let value =
                    resolve_file_source(cargo_path.clone(), original_path, is_release).await;
                if value.is_some() {
                    current_value.merge(value.unwrap());
                }
            }
        };
    }

    serde_json::from_value::<T>(current_value).unwrap()
}

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
            ConfigSource::File(original_path) => {
                let value = resolve_file_source(cargo_path.clone(), original_path, is_release);
                if let Some(v) = value {
                    current_value.merge(v);
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

/// Resolve files source to Serde [Value]
#[cfg(any(feature = "tokio", feature = "async-std"))]
async fn resolve_file_source(
    cargo_path: String,
    original_path: String,
    is_release: bool,
) -> Option<Value> {
    let path = match is_release {
        true => original_path.replace("{{env}}", "release"),
        false => original_path.replace("{{env}}", "debug"),
    };
    let path_buf = PathBuf::from(cargo_path.clone()).join(path.clone());
    if !path_buf.exists() {
        panic!("No file found in path: {}", path.clone());
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

#[cfg(not(any(feature = "tokio", feature = "async-std")))]
fn resolve_file_source(
    cargo_path: String,
    original_path: String,
    is_release: bool,
) -> Option<Value> {
    let path = match is_release {
        true => original_path.replace("{{env}}", "release"),
        false => original_path.replace("{{env}}", "debug"),
    };
    let path_buf = PathBuf::from(cargo_path).join(path.clone());
    if !path_buf.exists() {
        panic!("No file found in path: {}", path);
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

/// converts env vars to nested rust struct
fn process_envs(option: EnvSourceOptions) -> Result<Value, Error> {
    let temp_prefix = option.prefix.unwrap_or_else(|| "NORTH".to_string());
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
        let mut dot_key: String = String::new();
        for sub_keys in new_key.split(separator) {
            if dot_key.is_empty() {
                dot_key.push_str(sub_keys.to_case(case).as_str());
                dot_key.push('.');
            } else {
                dot_key.push_str(sub_keys.to_case(case).as_str());
            }
        }

        obj.dot_set(dot_key.as_str(), value).unwrap();
    }

    Ok(obj)
}

/// Read file content to serde [Value]s async
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

/// Read file content to serde [Value]s with blocking
///
#[cfg(not(any(feature = "tokio", feature = "async-std")))]
fn read_file_value(path: String) -> Value {
    let mut contents = String::new();
    let mut file = std::fs::File::open(path.clone()).expect("Unable to open file");
    file.read_to_string(&mut contents).unwrap();

    convert_str_to_value(path, contents)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
