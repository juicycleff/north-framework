#[cfg(feature = "api-poem")]
use {
    poem::{
        listener::TcpListener,
        middleware::{TokioMetrics, Tracing},
        EndpointExt,
    },
};

use crate::server::service::{NorthServiceBuilder, NorthService};
use north_common::utils::logger_utils::init_logger;

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
///         .api("/", Api)
///         .build();
///    power(service).start().await;
///    Ok(())
/// }
/// ```
///
pub struct North {
    pub(crate) service: NorthService,
}

/// Prepares the north api service
pub fn new_service() -> NorthServiceBuilder
{
    init_logger();
    NorthServiceBuilder::default()
}

/// implementation for `North` with `NorthService` integration
impl North {
    /// Prepares the north api service
    pub fn power(service: NorthService) -> North {
        init_logger();
        North {
            service,
        }
    }

    #[cfg(feature = "api-poem")]
    pub async fn up(self) -> std::io::Result<()> {
        let full_addr = format!(
            "{}:{}",
            self.service.options.address.clone().unwrap(),
            self.service.options.port.unwrap(),
        );

        let main_metrics = TokioMetrics::new();
        let app = self.service.poem_app;

        poem::Server::new(TcpListener::bind(full_addr))
            .run(app.at("/metrics/default", main_metrics.exporter()).with(Tracing))
            .await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
