# North Framework - A Microservice Framework for Rust
This repo contains packages and APIs that powers North Framework

[![Rust](https://github.com/Wakflo/north-framework/actions/workflows/rust.yml/badge.svg)](https://github.com/Wakflo/ngine/actions/workflows/rust.yml)

### Getting Started

#### Install Development Binaries
```Shell
sh setup.sh
``` 

#### Start Service
We use the Rust workspace and as such, we can start as service like the
ant service like below.
```Shell
cargo run --package ant --bin ant
``` 

#### Watch Service
We use the Rust workspace and as such, we can start as service like the
ant service like below.
```Shell
cargo watch -x '--package ant --bin ant'
``` 


#### Start Service with Docker
You can start a service using docker.
```Shell
docker compose up
``` 

#### Deploy
You can deploy to a Kubernetes cluster.
```Shell
cargo make deploy
``` 

### Folder Structure
The repository contains three important folders that requires mentioning, which are;

```text
Ngine
├── crates - In-house packages and libraries are managed
│   ├── north
│   ├── ngine-common
│   ├── ngine-funx
│   ├── ngine-datastore
│   └── ngine-task-runtime
├── services - All microservices and web APIs that make up Ngine
│   ├── ant
│   └── varni
├── tools - CLI tools and binaries required to maintain and run services in NgINE
├── infra - Infrastructure as code for Ngine
├── configs - Comprises shared configurations between all the services and packages
└── docs
```