#[cfg(feature = "api-actix")]
use actix_web::web::ServiceConfig;
#[cfg(feature = "db-arango")]
use aragog::DatabaseConnection;
use std::sync::Arc;
use yansi::Paint;
#[cfg(feature = "api-poem")]
use {poem::IntoEndpoint, poem::Route, poem_openapi::OpenApi as PoemOpenApi};

use north_common::registry::service_registry::ServiceRegistry;
use crate::server::north::North;
use crate::utils::boxed_connection::ArcArangoConnection;
use north_common::utils::logger_utils::print_format;

pub type BoxedServiceRegistry = Arc<dyn ServiceRegistry>;

/// A struct for service options. It holds the state for every created service
pub struct NorthServiceOptions {
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

/// Default implementation for NorthServiceOptions
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

/// NorthService struct for constructing a North service
pub struct NorthServiceBuilder<T>
where
    T: PoemOpenApi,
{
    pub(crate) options: NorthServiceOptions,

    #[cfg(feature = "api-poem")]
    pub poem_app: Route,

    #[cfg(feature = "api-poem")]
    pub(crate) api: Option<T>,

    #[cfg(feature = "db-arango")]
    pub(crate) db_connection: Option<ArcArangoConnection>,
}

impl<T> Default for NorthServiceBuilder<T>
where
    T: PoemOpenApi,
{
    fn default() -> Self {
        NorthServiceBuilder {
            options: Default::default(),
            #[cfg(feature = "api-poem")]
            poem_app: Route::new(),
            #[cfg(feature = "api-poem")]
            api: None,

            #[cfg(feature = "db-arango")]
            db_connection: None,
        }
    }
}

/// A trait for implementing north service
pub trait NorthServiceBuilderTrait<T>
where
    T: PoemOpenApi,
{
    // type Output;

    /// takes in a handler function
    #[cfg(feature = "api-poem")]
    fn handler<E>(self, path: impl AsRef<str>, ep: E) -> Self
    where
        E: IntoEndpoint,
        E::Endpoint: 'static;

    /// takes in a FnOnce(&mut web::ServiceConfig) function
    #[cfg(feature = "api-actix")]
    fn router(self, f: Box<dyn Fn(&mut ServiceConfig) + Send + Sync + 'static>) -> Self;

    /// takes in the version of the service
    fn version(self, version: &str) -> Self;

    /// takes in the name of the service
    fn name(self, name: &str) -> Self;

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
    fn api(self, api: T) -> Self;

    #[cfg(feature = "api-poem")]
    fn with_swagger(self, enable_swagger: bool) -> Self;

    /// Add a database connection to the state
    #[cfg(feature = "db-arango")]
    fn with_database(self, db_connection: Arc<DatabaseConnection>) -> Self;

    /// Enable auto SSL with lets encrypt acme
    #[cfg(feature = "api-poem")]
    fn with_auto_acme(self, enable_acme: bool) -> Self;

    fn service_registry(self, registry: BoxedServiceRegistry) -> Self;

    /// Used to pass state or context through to the handlers
    fn context<K: Send + Sync>(self, data: K) -> Self;

    /// Gracefully shutdown when the SIGTERM is called
    fn graceful_shutdown(self) -> Self;
}

impl<T> NorthServiceBuilder<T>
where
    T: PoemOpenApi,
{
    #[cfg(feature = "api-actix")]
    pub fn configure<F>(self, _f: F) -> Self
    where
        F: Fn(&mut ServiceConfig) + Send + Sync + 'static,
    {
        // self.router.borrow_mut().push(Box::new(f));
        self
    }

    pub fn up(self) -> North<T> {
        North { service: self }
    }
}

/// implement service trait for north service
impl<T: PoemOpenApi> NorthServiceBuilderTrait<T> for NorthServiceBuilder<T> {
    #[cfg(feature = "api-poem")]
    fn handler<E>(self, _path: impl AsRef<str>, _ep: E) -> Self
    where
        E: IntoEndpoint,
        E::Endpoint: 'static,
    {
        todo!()
    }

    #[cfg(feature = "api-actix")]
    fn router(self, _f: Box<dyn Fn(&mut ServiceConfig) + Send + Sync + 'static>) -> Self {
        todo!()
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

    fn api(mut self, api: T) -> Self {
        self.api = Some(api);
        self
    }

    fn with_swagger(mut self, enable_swagger: bool) -> Self {
        self.options.enable_swagger = enable_swagger;
        self
    }

    #[cfg(feature = "db-arango")]
    fn with_database(mut self, db_connection: Arc<DatabaseConnection>) -> Self {
        self.db_connection = Some(ArcArangoConnection {
            connection: db_connection,
        });
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

    fn context<K: Send + Sync>(self, _data: K) -> Self {
        // self.router_builder.data(data);
        self
    }

    fn graceful_shutdown(mut self) -> Self {
        self.options.graceful_shutdown = true;
        self
    }
}

pub fn print_server_info(opts: NorthServiceOptions) {
    println!(
        "{}",
        Paint::default("North Configuration")
            .bold()
            .underline()
            .blink()
    );
    print_format("name", opts.name.unwrap().as_str());
    print_format("version", opts.version.unwrap().as_str());
    print_format("address", opts.address.unwrap().as_str());
    print_format("port", opts.port.unwrap().to_string().as_str());
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
