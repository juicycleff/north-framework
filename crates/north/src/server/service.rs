#[cfg(feature = "api-actix")]
use actix_web::web::ServiceConfig;
#[cfg(feature = "db-arango")]
use aragog::DatabaseConnection;
use std::sync::Arc;
use yansi::Paint;
#[cfg(feature = "api-poem")]
use {
    poem::{IntoEndpoint, Route},
    poem_openapi::{OpenApi as PoemOpenApi, OpenApiService},
};

use crate::utils::boxed_connection::ArcArangoConnection;
use north_common::registry::service_registry::ServiceRegistry;
use north_common::utils::logger_utils::print_format;

pub type BoxedServiceRegistry = Arc<dyn ServiceRegistry>;

pub trait NorthApiTrait: PoemOpenApi + Sized + Clone {}

/// A struct for service options. It holds the state for every created service
#[derive(Clone)]
pub struct NorthServiceOptions {
    /// TCP address of servcer
    pub address: Option<String>,
    pub name: Option<String>,
    pub path_prefix: Option<String>,
    pub version: Option<String>,
    pub port: Option<u16>,
    pub graceful_shutdown: bool,
    pub enable_swagger: bool,
    pub auto_acme: bool,
    pub keep_alive: u32,
    pub read_timeout: u32,
    pub write_timeout: u32,
    pub registry: Option<BoxedServiceRegistry>,
}

/// default implementation for NorthServiceOptions
impl Default for NorthServiceOptions {
    fn default() -> Self {
        NorthServiceOptions {
            address: Some("127.0.0.1".to_string()),
            name: Some("service".to_string()),
            path_prefix: Some("/".to_string()),
            version: Some("latest".to_string()),
            port: Some(5000),
            graceful_shutdown: false,
            enable_swagger: false,
            auto_acme: false,
            keep_alive: 1,
            read_timeout: 2,
            write_timeout: 2,
            registry: None,
        }
    }
}


pub struct NorthService {
    pub options: Box<NorthServiceOptions>,

    #[cfg(feature = "api-poem")]
    pub poem_app: Box<Route>,
}

/// NorthService struct for constructing a North service
pub struct NorthServiceBuilder {
    pub(crate) options: Box<NorthServiceOptions>,

    #[cfg(feature = "api-poem")]
    pub(crate) poem_app: Box<Route>,

    #[cfg(feature = "api-poem")]
    pub(crate) custom_poem_app: Option<Box<Route>>,

    #[cfg(feature = "db-arango")]
    pub(crate) db_connection: Option<ArcArangoConnection>,
}

impl  Default for NorthServiceBuilder
{
    fn default() -> Self {
        NorthServiceBuilder {
            options: Box::new(Default::default()),

            #[cfg(feature = "api-poem")]
            poem_app: Box::new(Route::new()),

            #[cfg(feature = "api-poem")]
            custom_poem_app: None,

            #[cfg(feature = "db-arango")]
            db_connection: None,
        }
    }
}

/// A trait for implementing north service
pub trait NorthServiceBuilderTrait {
    /// takes in a handler function
    #[cfg(feature = "api-poem")]
    fn handler<E>(self, path: impl AsRef<str>, ep: E) ->  Self
    where
        E: IntoEndpoint,
        E::Endpoint: 'static;

    /// takes in the version of the service
    fn version(self, version: &str) ->  Self;

    /// takes in the name of the service
    fn name(self, name: &str) ->  Self;

    /// prefix to add to all route paths
    fn path_prefix(self, path: &str) -> Self;

    /// takes in the name of the service
    fn keep_alive(self, timeout: u32) -> Self;

    /// takes in the name of the service
    fn read_timeout(self, timeout: u32) -> Self;

    /// takes in the name of the service
    fn write_timeout(self, timeout: u32) -> Self;

    /// takes in the name of the service
    fn address(self, address: &str) -> Self;

    /// takes in the name of the service
    fn port(self, port: u16) -> Self;

    fn wrapper(self) -> Self;

    #[cfg(feature = "api-poem")]
    fn custom_http_server(self, app: Route) -> Self;

    #[cfg(feature = "api-poem")]
    fn api<T>(self, path: &str, api: T) -> Self
    where
        T: PoemOpenApi + Clone + 'static;

    fn with_swagger(self, enable_swagger: bool) -> Self;

    /// Add a database connection to the state
    #[cfg(feature = "db-arango")]
    fn with_database(self, db_connection: Arc<DatabaseConnection>) -> Self;

    /// Enable auto SSL with lets encrypt acme
    fn with_auto_acme(self, enable_acme: bool) -> Self;

    fn service_registry(self, registry: BoxedServiceRegistry) -> Self;

    /// Used to pass state or context through to the handlers
    #[cfg(feature = "api-poem")]
    fn data<T: Clone + Send + Sync + 'static>(self, data: T) -> Self;

    /// Gracefully shutdown when the SIGTERM is called
    fn graceful_shutdown(self) -> Self;

    fn build(self) -> NorthService;
}

impl NorthServiceBuilder {
    pub(crate) fn app_prefix(&self) -> String {
        let mut prefix = self.options.path_prefix.clone().unwrap();
        if prefix.starts_with('/') {
            prefix = prefix.strip_prefix('/').unwrap().to_string();
        };
        prefix
    }

    pub(crate) fn full_address(&self) -> String {
        let full_addr = format!(
            "{}:{}",
            self.options.address.clone().unwrap(),
            self.options.port.unwrap(),
        );
        format!("http://{}/{}", full_addr, self.app_prefix())
    }
}

/// implement service trait for north service
#[cfg(feature = "api-poem")]
impl NorthServiceBuilderTrait for NorthServiceBuilder {
    fn handler<E>(self, _path: impl AsRef<str>, _ep: E) -> Self
    where
        E: IntoEndpoint,
        E::Endpoint: 'static,
    {
        // self.poem_app.nest(path, ep);
        self
    }

    fn version(mut self, version: &str) -> Self {
        self.options.name = Some(version.to_string());
        self
    }

    fn name(mut self, name: &str) -> Self {
        self.options.name = Some(name.to_string());
        self
    }

    fn path_prefix(mut self, path: &str) -> Self {
        self.options.path_prefix = Some(path.to_string());
        self
    }

    fn keep_alive(mut self, timeout: u32) -> Self {
        self.options.keep_alive = timeout;
        self
    }

    fn read_timeout(mut self, timeout: u32) -> Self {
        self.options.read_timeout = timeout;
        self
    }

    fn write_timeout(mut self, timeout: u32) -> Self {
        self.options.write_timeout = timeout;
        self
    }

    fn address(mut self, address: &str) -> Self {
        self.options.address = Some(address.to_string());
        self
    }

    /// Port service runs on
    fn port(mut self, port: u16) -> Self {
        self.options.port = Some(port);
        self
    }

    fn wrapper(self) -> Self {
        todo!("Implement wrapper method")
    }

    fn custom_http_server(mut self, app: Route) -> Self {
        self.custom_poem_app = Some(Box::new(app));
        self
    }

    #[cfg(feature = "api-poem")]
    fn api<T>(mut self, path: &str, api: T) -> Self
        where
            T: PoemOpenApi + Clone + 'static
    {

        let title = self.options.name.as_ref().unwrap().clone();
        let version = self.options.version.as_ref().unwrap().clone();
        let api_service = OpenApiService::new(
            api,
            title,
            version,
        ).server(self.full_address());

        let ui = api_service.swagger_ui();
        let prefix = self.app_prefix();
        self.poem_app = Box::new(self.poem_app.nest(format!("{prefix}{}", path), api_service)
            .nest("/docs", ui));
        self
    }

    fn with_swagger(mut self, enable_swagger: bool) -> Self {
        self.options.enable_swagger = enable_swagger;
        self
    }

    #[cfg(feature = "db-arango")]
    fn with_database(mut self, db_connection: Arc<DatabaseConnection>) -> Self {
        self.db_connection = Some(ArcArangoConnection{connection: db_connection});
        self
    }

    fn with_auto_acme(mut self, enable_acme: bool) -> Self {
        self.options.auto_acme = enable_acme;
        self
    }

    /// adds a service registry
    fn service_registry(mut self, registry: BoxedServiceRegistry) -> Self {
        self.options.registry = Some(registry);
        self
    }

    #[cfg(feature = "api-poem")]
    fn data<T: Clone + Send + Sync + 'static>(self, _data: T) -> Self {
        // &self.poem_app.data(data);
        self
    }

    fn graceful_shutdown(mut self) -> Self {
        self.options.graceful_shutdown = true;
        self
    }

    fn build(self) -> NorthService {
        NorthService {
            options: self.options.clone(),
            poem_app: self.custom_poem_app.unwrap_or(self.poem_app)
        }
    }
}

#[allow(dead_code)]
pub(crate) fn print_server_info(opts: &NorthServiceOptions) {
    println!(
        "{}",
        Paint::default("North Configuration")
            .bold()
            .underline()
            .blink()
    );
    print_format("name", opts.name.as_ref().unwrap().as_str());
    print_format("version", opts.version.as_ref().unwrap().as_str());
    print_format("address", opts.address.as_ref().unwrap().as_str());
    print_format("port", opts.port.as_ref().unwrap().to_string().as_str());
    print_format("keep alive", opts.keep_alive.to_string().as_str());
    print_format(
        "read timeout",
        format!("{}{}", opts.read_timeout.to_string().as_str(), "s").as_str(),
    );
    print_format(
        "write timeout",
        format!("{}{}", opts.write_timeout.to_string().as_str(), "s").as_str(),
    );
    print_format(
        "write timeout",
        format!("{}{}", opts.write_timeout.to_string().as_str(), "s").as_str(),
    );
    if opts.graceful_shutdown {
        print_format("graceful shutdown", "enabled");
    } else {
        print_format("graceful shutdown", "disabled");
    }
}
