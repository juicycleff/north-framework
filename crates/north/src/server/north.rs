use poem::listener::Listener;
use poem::Route;

#[cfg(feature = "api-poem")]
use {
    poem::{
        listener::acme::{AutoCert, LETS_ENCRYPT_PRODUCTION},
        listener::TcpListener,
        middleware::{TokioMetrics, Tracing},
        EndpointExt,
    },
    poem_openapi::{OpenApi as PoemOpenAPi, OpenApiService},
};

use crate::server::service::{NorthServiceBuilder};
use crate::utils::boxed_connection::ArcArangoConnection;
use north_common::utils::logger_utils::init_logger;

/// ## North
/// HTTP and Websocket setup abstraction. It seeks to abstract away HTTP
/// adapters and framework something simpler in Wakflo
///
/// ### Example
/// ```rust
///
/// use poem_openapi::{payload::PlainText, OpenApi};
/// use north::power;
/// use north::NorthServiceBuilderTrait;
///
/// #[derive(Default)]
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
///     let north_app = power::<Api>()
///         .graceful_shutdown()
///         .address("localhost")
///         .name("Example Service")
///         .path_prefix("/api")
///         .port(8000)
///         .api(Api)
///         .up();
///    north_app.start();
///    Ok(())
/// }
/// ```
///
#[derive(Default)]
pub struct North<T>
where
    T: PoemOpenAPi,
{
    pub service: NorthServiceBuilder<T>,
}

/// Prepares the north api service
pub fn power<T>() -> NorthServiceBuilder<T>
where
    T: PoemOpenAPi,
{
    init_logger();
    NorthServiceBuilder::default()
}

/// Implementation for `North` with `NorthService` integration
impl<T: PoemOpenAPi + 'static> North<T> {
    #[cfg(feature = "api-poem")]
    pub async fn start(self) -> std::io::Result<()> {
        // prepare url
        let full_addr = format!(
            "{}:{}",
            self.service.options.address.clone().unwrap(),
            self.service.options.port.unwrap(),
        );
        let mut prefix = self.service.options.path_prefix.clone().unwrap();
        if prefix.starts_with('/') {
            prefix = prefix.strip_prefix('/').unwrap().to_string();
        };
        let docs_full_addr = format!("http://{}/{}", full_addr, prefix);

        let main_metrics = TokioMetrics::new();

        let api_service = OpenApiService::new(
            self.service.api.unwrap(),
            self.service
                .options
                .name
                .unwrap_or_else(|| "Docs".to_string()),
            self.service.options.version.unwrap(),
        )
        .server(docs_full_addr);

        if self.service.options.registry.is_some() {
            let reg = self.service.options.registry.unwrap();
            reg.register().await;
            // arc_runtime.block_on(reg.register());
            // self.service.poem_app.data::<BoxedServiceRegistry>(reg);
        }

        let ui = api_service.swagger_ui();

        if self.service.options.auto_acme {
            let auto_cert = AutoCert::builder()
                .directory_url(LETS_ENCRYPT_PRODUCTION)
                .domain(self.service.options.address.clone().unwrap())
                .build()?;

            return poem::Server::new(TcpListener::bind(full_addr).acme(auto_cert))
                .run(
                    self.service
                        .poem_app
                        .at("/metrics/default", main_metrics.exporter())
                        .nest(prefix, api_service)
                        .nest("/docs", ui)
                        .with(Tracing),
                )
                .await;
        }

        let app = self
            .service
            .poem_app
            .at("/metrics/default", main_metrics.exporter())
            .nest(prefix, api_service)
            .nest("/docs", ui);

        run_app(self.service.db_connection, app, full_addr).await
    }
}

#[cfg(feature = "db-arango")]
async fn run_app(
    db_connection: Option<ArcArangoConnection>,
    app: Route,
    full_addr: String,
) -> std::io::Result<()> {
    return match db_connection {
        None => {
            poem::Server::new(TcpListener::bind(full_addr))
                .run(app.with(Tracing))
                .await
        }
        Some(conn) => {
            poem::Server::new(TcpListener::bind(full_addr))
                .run(app.data(conn).with(Tracing))
                .await
        }
    };
}

#[cfg(not(feature = "db-arango"))]
async fn run_app(
    db_connection: Option<ArcArangoConnection>,
    app: Route,
    full_addr: String,
) -> std::io::Result<()> {
    poem::Server::new(TcpListener::bind(full_addr))
        .run(app.with(Tracing))
        .await
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
