## north-derives

[![Build Status](https://github.com/juicycleff/north-framework/actions/workflows/rust.yml/badge.svg)](https://github.com/juicycleff/north-framework/actions?query=branch%3Amaster)
[![](https://img.shields.io/crates/v/consul.svg)](https://crates.io/crates/north-consul)

Original Repo is here and all kudos to the owner [https://github.com/pierresouchay/consul-rust](https://github.com/pierresouchay/consul-rust)

[Documentation here](https://docs.rs/consul/).

Rust client libray for [Consul](http://consul.io/) HTTP API

### Usage

```
    extern crate north_consul;

    use std::collections::HashMap;
    use north_consul::{Client, Config, QueryMeta};
    use north_consul::catalog::Catalog;

    async fn main(){
        let config = Config::new().unwrap();
        let client = Client::new(config);
		let services: (HashMap<String, String>, QueryMeta) = client.services(None).await.unwrap();
		println!("{:?}", services);
    }
```


For more examples, see the **[tests](https://github.com/stusmall/consul-rust/blob/master/tests)** .

### Installation

Simply include the consul-rust in your Cargo dependencies.

```
[dependencies]
north_consul = "0.0.1"
```
