mod routes;
use crate::routes::Api;
use north::NorthServiceBuilderTrait;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    //#region Setup Server
    let north_app = north::power::<Api>()
        .graceful_shutdown()
        .address("localhost")
        .name("Basic App")
        .path_prefix("/api")
        .port(8000)
        .api(Api)
        .up();

    north_app.start().await
    //#endregion
}
