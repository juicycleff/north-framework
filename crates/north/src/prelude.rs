pub use std::cell::RefCell;
pub use std::rc::Rc;
pub use itertools::{Itertools};
pub use std::sync::{Arc, Mutex, MutexGuard};
#[cfg(feature = "db-arango")]
pub use aragog::DatabaseConnection;
#[cfg(feature = "api-poem")]
pub use {
    poem::{IntoEndpoint, Route, EndpointExt},
    poem_openapi::{OpenApi as PoemOpenApi, OpenApiService},
};
#[cfg(feature = "db-arango")]
pub use crate::utils::boxed_connection::ArcArangoConnection;
pub use north_common::{
    registry::service_registry::ServiceRegistry,
    state::NorthStateData,
};
pub use crate::contracts::*;
pub use crate::service::*;

pub type BoxedServiceRegistry = Arc<dyn ServiceRegistry>;