use crate::registry::service_instance_state::ServiceInstanceState;
use crate::registry::service_status_constants::ServiceStatus;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct IService {
    pub name: String,
    pub id: String,
    pub port: u32,

    pub region: Option<String>,
    pub zone: Option<String>,

    pub host: String,
    pub secure: bool,
    pub metadata: Option<HashMap<String, String>>,
    pub tag: Option<Vec<String>>,
    pub state: ServiceInstanceState,
}

/// #### ServiceInstance
#[derive(Default, Clone)]
pub struct ServiceInstanceOptions {
    ///instance_id the id of the instance.
    pub instance_id: String,

    ///Node the id of the instance.
    pub node_id: Option<String>,

    ///service_id the id of the service.
    pub service_id: String,

    ///host where the service instance can be found.
    pub host: String,

    ///service instance status.
    pub status: ServiceStatus,

    ///service tags.
    pub tags: Option<Vec<String>>,

    ///port the port on which the service is running.
    pub port: u32,

    ///secure indicates whether or not the connection needs to be secure.
    pub secure: bool,

    ///node instance state.
    pub state: Option<ServiceInstanceState>,

    ///metadata optional a map containing metadata.
    pub metadata: Option<HashMap<String, String>>,
}

/// #### ServiceInstance
/// Struct for service instance
pub trait ServiceInstance {
    /// Returns the unique instance ID as registered.
    fn get_instance_id(&self) -> String;

    /// Returns the service ID as registered
    fn get_service_id(&self) -> String;

    /// Returns hostname of the registered service
    fn get_host(&self) -> String;

    /// Returns port of the registered service
    fn get_port(&self) -> u32;

    /// Returns whether the registered service is secure
    fn is_secure(&self) -> bool;

    /// Returns service universal resource identifier
    fn get_uri(&self) -> String;

    /// Returns the scheme of the service.
    fn get_scheme(&self) -> String;

    /// Returns the key / value pair associated with the service id.
    fn get_metadata(&self) -> HashMap<String, String>;

    /// Returns the key / value pair associated with the service id.
    fn get_tags(&self) -> Vec<String>;

    /// Returns returns service instance health status.
    fn get_status(&self) -> String;

    /// Returns service instance cluster node ID
    fn get_node_id(&self) -> String;

    /// Returns service instance state
    fn get_state(self) -> ServiceInstanceState;
}
