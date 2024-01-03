pub use crate::contracts::*;
pub use crate::service::*;
#[cfg(feature = "db-arango")]
pub use crate::utils::boxed_connection::ArcArangoConnection;
#[cfg(feature = "db-arango")]
pub use aragog::DatabaseConnection;
pub use itertools::Itertools;
pub use north_common::{registry::service_registry::ServiceRegistry, state::NorthStateData};
pub use std::cell::RefCell;
pub use std::rc::Rc;
pub use std::sync::{Arc, Mutex, MutexGuard};
#[cfg(feature = "api-poem")]
pub use {
    poem::{EndpointExt, IntoEndpoint, Route},
    poem_openapi::{OpenApi as PoemOpenApi, OpenApiService},
};

pub type BoxedServiceRegistry = Arc<dyn ServiceRegistry>;
