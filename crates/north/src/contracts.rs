use crate::prelude::*;

#[cfg(feature = "api-poem")]
pub trait NorthApiTrait: PoemOpenApi + Sized + Clone {}

/// A trait for implementing north service
pub trait NorthServiceBuilderTrait<T> {
    /// takes in a handler function
    #[cfg(feature = "api-poem")]
    fn handler<E>(self, path: impl AsRef<str>, ep: E) -> Self
    where
        E: IntoEndpoint,
        E::Endpoint: 'static;

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
    fn custom_http_server(self, app: Route) -> Self;

    #[cfg(feature = "api-poem")]
    fn controller(self, api: T) -> Self;

    fn with_swagger(self, enable_swagger: bool) -> Self;

    /// Add a database connection to the state
    #[cfg(feature = "db-arango")]
    fn with_database(self, db_connection: Arc<DatabaseConnection>) -> Self;

    /// Enable auto SSL with lets encrypt acme
    fn with_auto_acme(self, enable_acme: bool) -> Self;

    fn service_registry(self, registry: BoxedServiceRegistry) -> Self;

    /// Used to pass state or context through to the handlers
    #[cfg(feature = "api-poem")]
    fn with_data<S: NorthStateData + Send + Sync + 'static>(self, data: S) -> Self;

    /// Gracefully shutdown when the SIGTERM is called
    fn graceful_shutdown(self) -> Self;

    fn build(&mut self) -> NorthService;
}
