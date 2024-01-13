mod routes;

use serde_this_or_that::{as_i64};
use crate::routes::{Api, SecondApi};
use north::{NorthServiceBuilderTrait, NorthStateData};
use poem_openapi::__private::serde::{Deserialize, Serialize};

#[derive(Clone)]
struct TestData {
    pub title: String,
}

impl NorthStateData for TestData {}

#[derive(Clone)]
struct TestDataMod {
    pub title: String,
}

impl NorthStateData for TestDataMod {}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ExampleConfig {
    pub host: String,
    #[serde(deserialize_with = "as_i64")]
    pub port: i64
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let env_cfg = north_config::EnvSourceOptions {
        prefix: Some("NORTH_".to_string()),
        ..std::default::Default::default()
    };

    let config_options = north_config::NorthConfigOptions {
        sources: vec![
            north_config::ConfigSource::File(
                "../configs/bootstrap.{{env}}.yaml".to_string(),
                Some(north_config::FileSourceOptions {
                    skip_on_error: true,
                    ..std::default::Default::default()
                }),
            ),
            north_config::ConfigSource::Env(env_cfg),
        ],
    };
    let north_config = north_config::new_config::<ExampleConfig>(config_options).await;
    let config = north_config.get_value();

    //#region Setup Server
    let service = north::new_service()
        .graceful_shutdown()
        .address("127.0.0.1")
        .name("Basic App")
        .path_prefix("/api")
        .port(8000)
        .with_data::<TestData>(TestData {
            title: "Wow".to_string(),
        })
        .with_data::<TestDataMod>(TestDataMod {
            title: "Wow".to_string(),
        })
        .controller((Api, SecondApi))
        .build();

    north::power(service).up().await
    //#endregion
}
