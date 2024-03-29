<h1 align="center" style="text-underline: none"> North Framework </h1>
<h4 align="center"> A Microservice Framework for Rust </h4>
<h6 align="center">This repo contains packages and APIs that powers North Framework</h6>

<div align="center">
  <a href="https://github.com/juicycleff/north-framework/actions/workflows/rust">
    <img src="https://github.com/juicycleff/north-framework/actions/workflows/rust.yml/badge.svg" />
  </a>

  <a href="https://blog.rust-lang.org/2021/11/01/Rust-1.61.0.html">
    <img src="https://img.shields.io/badge/rustc-1.61.0+-ab6000.svg"
      alt="rustc 1.61.0+" />
  </a>

  <a href="https://deps.rs/repo/github/juicycleff/north-framework">
    <img src="https://img.shields.io/librariesio/release/cargo/north.svg" />
  </a>
</div>

<p align="center"> The project was born out of the need to build microsevices with Rust, benefiting
from the performance and sweetness bundled in Rust. North framework avoids building transport layers itself,
but more so uses existing crates such as web frameworks like <a href="https://github.com/poem-web/poem" target="_blank">poem</a>, to make a simple, flexible,
performant and easy to use microservice framework.
</p>

***

### Getting Started

```rust
/// Entry into Example service
#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    //#region Setup Server
    let service = north::new_service()
        .graceful_shutdown()
        .address("localhost")
        .name("Example Service")
        .path_prefix("/api")
        .port(8000)
        .api("api", Api)
        .build();

    north::power(service).up().await
    //#endregion
}
```

### Crates
| Crate                                                                                                         | Description              | Documentation                      | ChangeLog                                  |
|---------------------------------------------------------------------------------------------------------------|--------------------------|------------------------------------|--------------------------------------------|
| **north** [![](https://img.shields.io/crates/v/north)](https://crates.io/crates/north)                         | North Framework          | [(README)](crates/north/README.md) | [(CHANGELOG)](crates/north/CHANGELOG.md)           |
| **north-config** [![](https://img.shields.io/crates/v/north-config)](https://crates.io/crates/north-config)    | North dynamic config     | [(README)](crates/north-config/README.md) | [(CHANGELOG)](crates/north-config/CHANGELOG.md)    |
| **north-service** [![](https://img.shields.io/crates/v/north-service)](https://crates.io/crates/north-service) | [WIP] Service Reg & Disc | [(README)](crates/north-service/README.md) | [(CHANGELOG)](crates/north-service/CHANGELOG.md)   |
| **north-consul** [![](https://img.shields.io/crates/v/north-consul)](https://crates.io/crates/north-consul) | Async consul client      | [(README)](crates/north-service/README.md) | [(CHANGELOG)](crates/north-service/CHANGELOG.md) |

***

### Folder Structure
The repository contains folders that require mentioning, which are;

```text
North Framework
├── crates - In-house packages and libraries are managed
│   ├── north
│   ├── north-config
│   ├── north-service
│   ├── north-common
│   ├── north-consul
├── example - north examples
│   ├── basic
└── docs
```

### Develop

#### Install Development Binaries
```Shell
sh setup.sh
```