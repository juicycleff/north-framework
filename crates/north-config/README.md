# North Config
North config is a multi source configuration crate designed as part of the
North Microservice Framework. It supports multiple source which are merged into one config as the final sources,
with first in first applied directive merge pattern, meaning last config source will override
the others.

Currently, North Config supports only local data sources with plans for supporting remote sources.
Supported sources are `json`, `yaml`, `ron`, `toml`, and `env variables`.

Nesting or nested objects are supported for all source types

Supports `async-std`, `tokio`, with `std` as default

### Getting Started

```rust
use north_config::{ConfigSource, EnvSourceOptions, NorthConfigOptions};

#[derive(Clone, serde::Deserialize, Debug)]
struct DemoConfig {
    pub host: Option<String>,
}

///#[tokio::main]
fn main() {
    let config_options = NorthConfigOptions {
        sources: vec![
            // ConfigSource::File("examples/configs/bootstrap.{{env}}.yaml".to_string()),
            ConfigSource::Env(EnvSourceOptions::default()),
        ],
    };
    let config = north_config::new_config::<DemoConfig>(config_options).await;
    let config = config.get_value();
    
    Ok()
}
```