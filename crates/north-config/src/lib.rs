#![allow(dead_code)]
extern crate core;

mod config_source;
mod error;
mod utils;

pub mod serde_utils;
pub(crate) mod watcher;

pub use {
    self::config_source::{
        new_config, Case, ConfigSource, CustomConfigSource, EnvSourceOptions, FileSourceOptions,
        NorthConfig, NorthConfigOptions,
    },
    self::error::Error,
};

#[cfg(test)]
mod tests {
    use crate::{ConfigSource, EnvSourceOptions, NorthConfigOptions};

    #[derive(Clone, serde::Deserialize, Debug)]
    struct NestedConfig {
        pub foo: String,
        pub bar: String,
    }

    #[derive(Clone, serde::Deserialize, Debug)]
    struct DemoConfig {
        pub host: Option<String>,
        pub nested: NestedConfig,
    }

    #[derive(Clone, serde::Deserialize, Debug)]
    struct SimpleDemoConfig {
        pub host: Option<String>,
    }

    fn setup_env() {
        destroy_env();
        envmnt::set("NORTH_HOST", "address");
        envmnt::set("NORTH_NESTED__FOO", "env foo");
        envmnt::set("NORTH_NESTED__BAR", "env bar");
    }

    fn destroy_env() {
        envmnt::remove("NORTH_HOST");
        envmnt::remove("NORTH_NESTED__FOO");
        envmnt::remove("NORTH_NESTED__BAR");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn reads_config_from_file_with_release_mode() {
        let config_options = NorthConfigOptions {
            sources: vec![ConfigSource::File(
                "../../examples/configs/test.{{env}}.json".to_string(),
            )],
        };
        let config = crate::new_config::<DemoConfig>(config_options).await;
        let config = config.get_value().clone();
        let host = config.host.unwrap();
        assert_eq!(host, "0.0.0.245");
    }

    #[cfg(feature = "async-std")]
    #[async_std::test]
    async fn reads_config_from_file_with_release_mode() {
        let config_options = NorthConfigOptions {
            sources: vec![ConfigSource::File(
                "../../examples/configs/test.{{env}}.json".to_string(),
            )],
        };
        let config = crate::new_config::<DemoConfig>(config_options).await;
        let config = config.get_value().clone();
        let host = config.host.unwrap();
        assert_eq!(host, "0.0.0.245");
    }

    #[cfg(not(any(feature = "tokio", feature = "async-std")))]
    #[test]
    fn reads_config_from_file_with_release_mode() {
        let config_options = NorthConfigOptions {
            sources: vec![ConfigSource::File(
                "../../examples/configs/test.{{env}}.json".to_string(),
            )],
        };
        let config = crate::new_config::<DemoConfig>(config_options);
        let config = config.get_value().clone();
        assert_eq!(config.host.unwrap(), "0.0.0.245");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn reads_config_from_file() {
        let config_options = NorthConfigOptions {
            sources: vec![ConfigSource::File(
                "../../examples/configs/test.debug.json".to_string(),
            )],
        };
        let config = crate::new_config::<DemoConfig>(config_options).await;
        let config = config.get_value().clone();
        let host = config.host.unwrap();
        assert_eq!(host, "0.0.0.245");
    }

    #[cfg(feature = "async-std")]
    #[async_std::test]
    async fn reads_config_from_file() {
        let config_options = NorthConfigOptions {
            sources: vec![ConfigSource::File(
                "../../examples/configs/test.debug.json".to_string(),
            )],
        };
        let config = crate::new_config::<DemoConfig>(config_options).await;
        let config = config.get_value().clone();
        let host = config.host.unwrap();
        assert_eq!(host, "0.0.0.245");
    }

    #[cfg(not(any(feature = "tokio", feature = "async-std")))]
    #[test]
    fn reads_config_from_file() {
        let config_options = NorthConfigOptions {
            sources: vec![ConfigSource::File(
                "../../examples/configs/test.debug.json".to_string(),
            )],
        };
        let config = crate::new_config::<DemoConfig>(config_options);
        let config = config.get_value().clone();
        assert_eq!(config.host.unwrap(), "0.0.0.245");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn merge_and_overwrite_duplicate_keys_from_two_config_file() {
        let config_options = NorthConfigOptions {
            sources: vec![
                ConfigSource::File("../../examples/configs/test.debug.json".to_string()),
                ConfigSource::File("../../examples/configs/test.release.json".to_string()),
            ],
        };
        let config = crate::new_config::<DemoConfig>(config_options).await;
        let config = config.get_value().clone();
        let host = config.host.unwrap();
        assert_eq!(host, "0.0.0.0");
    }

    #[cfg(not(any(feature = "tokio", feature = "async-std")))]
    #[test]
    fn merge_and_overwrite_duplicate_keys_from_two_config_file() {
        let config_options = NorthConfigOptions {
            sources: vec![
                ConfigSource::File("../../examples/configs/test.debug.json".to_string()),
                ConfigSource::File("../../examples/configs/test.release.json".to_string()),
            ],
        };
        let config = crate::new_config::<DemoConfig>(config_options);
        let config = config.get_value().clone();
        assert_eq!(config.host.unwrap(), "0.0.0.0");
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn merge_two_files_from_json_and_yaml_sources() {
        let config_options = NorthConfigOptions {
            sources: vec![
                ConfigSource::File("../../examples/configs/test.release.json".to_string()),
                ConfigSource::File("../../examples/configs/test.release.yaml".to_string()),
            ],
        };
        let config = crate::new_config::<DemoConfig>(config_options);
        let config = config.get_value().clone();
        let host = config.host.unwrap();
        assert_eq!(host, "127.0.0.1");
    }

    #[cfg(all(feature = "yaml", feature = "toml"))]
    #[test]
    fn merge_three_files_from_json_toml_and_yaml_sources() {
        let config_options = NorthConfigOptions {
            sources: vec![
                ConfigSource::File("../../examples/configs/test.release.json".to_string()),
                ConfigSource::File("../../examples/configs/test.release.yaml".to_string()),
                ConfigSource::File("../../examples/configs/test.release.toml".to_string()),
            ],
        };
        let config = crate::new_config::<DemoConfig>(config_options);
        let config = config.get_value().clone();
        let host = config.host.unwrap();
        assert_eq!(host, "test");
    }

    #[test]
    fn merge_deep_nested_objects() {
        let config_options = NorthConfigOptions {
            sources: vec![
                ConfigSource::File("../../examples/configs/test.release.json".to_string()),
                ConfigSource::File("../../examples/configs/test.debug.json".to_string()),
            ],
        };
        let config = crate::new_config::<DemoConfig>(config_options);
        let config = config.get_value().clone();
        let host = config.host.unwrap();
        let nested = config.nested;

        assert_eq!(host, "0.0.0.245");
        assert_eq!(nested.foo, "Well its foo");
    }

    #[test]
    fn merge_env_and_file_sources() {
        destroy_env();
        envmnt::set("NORTH_HOST", "address");
        envmnt::set("NORTH_NESTED__FOO", "env foo");

        let config_options = NorthConfigOptions {
            sources: vec![
                ConfigSource::File("../../examples/configs/test.release.json".to_string()),
                ConfigSource::File("../../examples/configs/test.debug.json".to_string()),
                ConfigSource::Env(EnvSourceOptions::default()),
            ],
        };
        let config = crate::new_config::<DemoConfig>(config_options);
        let config = config.get_value().clone();
        let host = config.host.unwrap();
        let nested = config.nested;

        assert_eq!(host, "address");
        assert_eq!(nested.foo, "env foo");
        assert_eq!(nested.bar, "env bar");

        destroy_env();
    }

    #[test]
    fn read_deep_config_from_env_sources_default_prefix() {
        setup_env();

        let config_options = NorthConfigOptions {
            sources: vec![ConfigSource::Env(EnvSourceOptions::default())],
        };
        let config = crate::new_config::<DemoConfig>(config_options);
        let config = config.get_value().clone();
        let host = config.host.unwrap();
        let nested = config.nested;

        assert_eq!(host, "address");
        assert_eq!(nested.foo, "env foo");
        assert_eq!(nested.bar, "env bar");

        destroy_env();
    }

    #[test]
    fn read_deep_config_from_env_sources_custom_prefix() {
        envmnt::set("TESTNORTH_HOST", "address");
        envmnt::set("TESTNORTH_NESTED__FOO", "env foo 2");
        envmnt::set("TESTNORTH_NESTED__BAR", "env bar 2");

        let mut env_opts = EnvSourceOptions::default();
        env_opts.prefix = Some("TESTNORTH".to_string());

        let config_options = NorthConfigOptions {
            sources: vec![ConfigSource::Env(env_opts)],
        };

        let config = crate::new_config::<DemoConfig>(config_options);
        let config = config.get_value().clone();
        let host = config.host.unwrap();
        let nested = config.nested;

        assert_eq!(host, "address");
        assert_eq!(nested.foo, "env foo 2");
        assert_eq!(nested.bar, "env bar 2");

        envmnt::remove("TESTNORTH_HOST");
        envmnt::remove("TESTNORTH_NESTED__FOO");
        envmnt::remove("TESTNORTH_NESTED__BAR");
    }

    #[cfg(feature = "ron")]
    #[test]
    fn read_config_from_ron_source() {
        let config_options = NorthConfigOptions {
            sources: vec![ConfigSource::File(
                "../../examples/configs/test.release.ron".to_string(),
            )],
        };
        let config = crate::new_config::<DemoConfig>(config_options);
        let config = config.get_value().clone();
        let host = config.host.unwrap();

        assert_eq!(host, "ron-host");
    }

    // #[test]
    // fn read_config_from_ron_source() {
    //     let config_options = NorthConfigOptions {
    //         sources: vec![
    //             ConfigSource::File("../../examples/configs/test.release.yaml".to_string()),
    //         ],
    //     };
    //
    //     assert_eq!(crate::new_config::<SimpleDemoConfig>(config_options), "missing yaml feature for crate, please enable yaml feature", &(), None);
    // }
}
