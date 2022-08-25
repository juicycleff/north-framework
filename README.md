# North Framework - A Microservice Framework for Rust
This repo contains packages and APIs that powers North Framework

[![Rust](https://github.com/Wakflo/north-framework/actions/workflows/rust.yml/badge.svg)](https://github.com/Wakflo/north-framework/actions/workflows/rust.yml)

### Getting Started

#### Install Development Binaries
```Shell
sh setup.sh
```

#### Deploy
You can deploy to a Kubernetes cluster.
```Shell
cargo make deploy
``` 

### Folder Structure
The repository contains folders that requires mentioning, which are;

```text
North Framework
├── crates - In-house packages and libraries are managed
│   ├── north
│   ├── north-config
└── docs
```