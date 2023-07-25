mod routes;
use crate::routes::{Api, SecondApi};
use north::{NorthServiceBuilderTrait, NorthStateData};

#[derive(Clone)]
struct TestData {
    pub title: String
}

impl NorthStateData for TestData {
}

#[derive(Clone)]
struct TestDataMod {
    pub title: String
}

impl NorthStateData for TestDataMod {
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    //#region Setup Server
    let service = north::new_service()
        .graceful_shutdown()
        .address("127.0.0.1")
        .name("Basic App")
        .path_prefix("/api")
        .port(8000)
        .with_data::<TestData>(TestData{ title: "Wow".to_string() })
        .with_data::<TestDataMod>(TestDataMod{ title: "Wow".to_string() })
        .controller((Api, SecondApi))
        .build();

    north::power(service).up().await
    //#endregion
}
