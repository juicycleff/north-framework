#[cfg(feature = "api-poem")]
use {
    poem::{
        listener::TcpListener,
        middleware::{TokioMetrics, Tracing, AddData},
        EndpointExt,
    },
};
use crate::service::{NorthServiceBuilder, NorthService};
use north_common::utils::logger_utils::init_logger;

#[cfg(feature = "db-arango")]
use north_derives::process_poem;

/// ## North
/// HTTP and Websocket setup abstraction. It seeks to abstract away HTTP
/// adapters and framework something simpler in Wakflo
///
/// ### Example
/// ```rust
///
/// use poem_openapi::{payload::PlainText, OpenApi};
/// use north::{new_service};
/// use north::NorthServiceBuilderTrait;
///
/// #[derive(Default, Clone)]
/// pub struct Api;
///
/// #[OpenApi]
/// impl Api {
///     #[oai(path = "/", method = "get")]
///     async fn get_index(&self) -> PlainText<String> {
///         PlainText("Hello Example Service".to_string())
///     }
/// }
///
///#[tokio::main]
///pub async fn main() -> std::io::Result<()> {
///     // _server.await; // should uncomment this line
///     let service = new_service()
///         .graceful_shutdown()
///         .address("localhost")
///         .name("Example Service")
///         .path_prefix("/api")
///         .port(8000)
///         .controller("/", Api)
///         .build();
///    north::power(service).up();
///    Ok(())
/// }
/// ```
///
pub struct North {
    pub(crate) service: NorthService,
}

/// Prepares the north api service
#[cfg(feature = "api-poem")]
pub fn new_service<T>() -> NorthServiceBuilder<T> where T: poem_openapi::OpenApi + Clone + 'static
{
    init_logger();
    NorthServiceBuilder::default()
}

/// Prepares the north api service
pub fn power(service: NorthService) -> North {
    init_logger();
    North {
        service,
    }
}

/// implementation for `North` with `NorthService` integration
impl North {
    #[cfg(all(feature = "api-native", not(feature = "api-poem")))]
    pub async fn up(self) -> std::io::Result<()> {
        crate::server::start_server(&self.service.options).await.unwrap();
        Ok(())
    }

    // #[cfg(all(feature = "api-poem", not(feature = "api-native")))]
    pub async fn up(self) -> std::io::Result<()> {
        let full_addr = format!(
            "{}:{}",
            self.service.options.address.clone().unwrap(),
            self.service.options.port.unwrap(),
        );

        let main_metrics = TokioMetrics::new();
        let app = self.service.poem_app;
        let end = app.at("/metrics/default", main_metrics.exporter()).with(Tracing);
        let state = self.service.state_data_list.as_slice();
        let d = [..state];

        poem::Server::new(TcpListener::bind(full_addr))
            .run(process_poem!(end, ["2", "5"]))
            .await
    }

}