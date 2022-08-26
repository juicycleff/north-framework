# North Framework - A Microservice Framework for Rust
This repo contains packages and APIs that powers North Framework

[![Rust](https://github.com/juicycleff/north-framework/actions/workflows/rust.yml/badge.svg)](https://github.com/juicycleff/north-framework/actions/workflows/rust.yml)

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

### Folder Structure
The repository contains folders that requires mentioning, which are;

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