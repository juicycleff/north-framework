<h1 align="center"> North Framework </h1>
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
and uses web frameworks like <a href="https://github.com/poem-web/poem" target="_blank"> poem </a>, but more so uses existing crates to make a simple, flexible,
performant and easy to use microservice framework.
</p>

***

### Getting Started

```rust
/// Entry into Example service
#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    //#region Setup Server
    let north_app = power::<Api>()
        .graceful_shutdown()
        .address("localhost")
        .name("Example Service")
        .path_prefix("/api")
        .port("8080")
        .api(Api)
        .up();

    north_app.start().await
    //#endregion
}
```

### Crates
| Crate                                                                                                         | Description                    | Documentation                        | ChangeLog                                  |
|---------------------------------------------------------------------------------------------------------------|--------------------------------|--------------------------------------|--------------------------------------------|
| **north** [![](https://img.shields.io/crates/v/north)](https://crates.io/crates/north)                         | North Framework                | [(README)](north/README.md)           | [(CHANGELOG)](north/CHANGELOG.md)           |
| **north-config** [![](https://img.shields.io/crates/v/north-config)](https://crates.io/crates/north-config)    | North dynamic config           | [(README)](north-config/README.md)    | [(CHANGELOG)](north-config/CHANGELOG.md)    |
| **north-service** [![](https://img.shields.io/crates/v/north-service)](https://crates.io/crates/north-service) | Service discovery and registry | [(README)](north-service/README.md)   | [(CHANGELOG)](north-service/CHANGELOG.md)   |
| **north-consul** [![](https://img.shields.io/crates/v/north-consul)](https://crates.io/crates/north-consul) | Async consul client            | [(README)](north-service/README.md) | [(CHANGELOG)](north-service/CHANGELOG.md) |

***

### Folder Structure
The repository contains folders that require mentioning, which are;

```text
North Framework
├── crates - In-house packages and libraries are managed
│   ├── north
│   ├── north-config
├── example - north examples
│   ├── basic
└── docs
```

### Develop

#### Install Development Binaries
```Shell
sh setup.sh
```