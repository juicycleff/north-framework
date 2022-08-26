use crate::registry::service_instance::ServiceInstance;
use std::sync::Arc;

/// #### ServicePool
#[allow(dead_code)]
pub struct ServicePool {
    /// services in the pool
    _services: Vec<Arc<dyn ServiceInstance>>,
}
