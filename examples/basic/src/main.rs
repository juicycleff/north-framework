mod routes;
use crate::routes::Api;
use north::NorthServiceBuilderTrait;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    //#region Setup Server
    let service = north::new_service()
        .graceful_shutdown()
        .address("localhost")
        .name("Basic App")
        .path_prefix("/api")
        .port(8000)
        .api("api", Api)
        .build();

    north::North::power(service).up().await
    //#endregion
}
