# North Framework
A microservice framework

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