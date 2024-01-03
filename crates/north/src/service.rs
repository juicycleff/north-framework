use crate::prelude::*;

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

    pub state_data_list: Vec<Box<dyn NorthStateData>>,

    #[cfg(feature = "api-poem")]
    pub poem_app: Box<Route>,
}

/// NorthService struct for constructing a North service
pub struct NorthServiceBuilder<T>
where
    T: PoemOpenApi + Clone + 'static,
{
    pub(crate) options: Box<NorthServiceOptions>,

    pub(crate) state_data_list: Vec<Box<dyn NorthStateData>>,

    #[cfg(feature = "api-poem")]
    pub(crate) poem_app: Route,

    #[cfg(feature = "api-poem")]
    pub(crate) custom_poem_app: Option<Box<Route>>,

    #[cfg(feature = "api-poem")]
    pub(crate) apis: Option<T>,

    #[cfg(feature = "db-arango")]
    pub(crate) db_connection: Option<ArcArangoConnection>,
}

impl<T> Default for NorthServiceBuilder<T>
where
    T: PoemOpenApi + Clone + 'static,
{
    fn default() -> Self {
        NorthServiceBuilder {
            options: Box::new(Default::default()),

            state_data_list: vec![],

            #[cfg(feature = "api-poem")]
            poem_app: Route::new(),

            #[cfg(feature = "api-poem")]
            custom_poem_app: None,

            apis: None,

            #[cfg(feature = "db-arango")]
            db_connection: None,
        }
    }
}

impl<T> NorthServiceBuilder<T>
where
    T: PoemOpenApi + Clone + 'static,
{
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
impl<T> NorthServiceBuilderTrait<T> for NorthServiceBuilder<T>
where
    T: PoemOpenApi + Clone + 'static,
{
    #[cfg(feature = "api-poem")]
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

    #[cfg(feature = "api-poem")]
    fn custom_http_server(mut self, app: Route) -> Self {
        self.custom_poem_app = Some(Box::new(app));
        self
    }

    #[cfg(feature = "api-poem")]
    fn controller(mut self, api: T) -> Self {
        self.apis = Some(api);
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

    #[cfg(feature = "api-poem")]
    fn with_data<S: NorthStateData + Send + Sync + 'static>(mut self, data: S) -> Self {
        self.state_data_list.push(Box::new(data));
        self
    }

    fn graceful_shutdown(mut self) -> Self {
        self.options.graceful_shutdown = true;
        self
    }

    #[cfg(feature = "api-poem")]
    fn build(&mut self) -> NorthService {
        // let poem_app = Route::new();
        let title = self.options.name.as_ref().unwrap().clone();
        let version = self.options.version.as_ref().unwrap().clone();

        let api_service = OpenApiService::new(self.apis.clone().unwrap(), title, version)
            .server(self.full_address());

        let ui = api_service.swagger_ui();
        let prefix = self.app_prefix();

        let c_app = std::mem::take::<Option<Box<Route>>>(&mut self.custom_poem_app);
        let def_app = std::mem::take::<Route>(&mut self.poem_app);

        // println!(" len = {}", self.state_data_list.len());

        NorthService {
            options: self.options.clone(),
            state_data_list: self.state_data_list.clone(),
            poem_app: c_app.unwrap_or(Box::new(
                def_app
                    .nest(format!("/{prefix}"), api_service)
                    .nest("/docs", ui),
            )),
        }
    }

    #[cfg(all(feature = "api-native", not(feature = "api-poem")))]
    fn build(mut self) -> NorthService {
        NorthService {
            options: self.options.clone(),
        }
    }
}
