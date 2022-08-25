use crate::discovery::discovery_options::DiscoveryOptions;
use crate::discovery::heartbeat::HeartbeatOptions;
use crate::registry::registration::Registration;
use crate::registry::service_instance::ServiceInstance;
use crate::registry::service_registry::ServiceRegistry;
use std::collections::HashMap;

pub trait ServiceRegistryBuilder<R, K, S>
where
    R: ServiceRegistry,
    S: ServiceInstance,
    K: Registration<S>,
{
    fn heartbeat_options(options: HeartbeatOptions) -> Self;

    fn build() -> R;
}

pub trait RegistrationBuilder<R, S>
where
    R: Registration<S>,
{
    fn service_name(self, name: String) -> Self;

    fn tags(self, tags: Vec<String>) -> Self;

    fn instance_id(self, id: String) -> Self;

    fn host(self, host: String) -> Self;

    fn port(self, port: u32) -> Self;

    fn version(self, version: String) -> Self;

    fn status(self, status: String) -> Self;

    fn metadata(self, metadata: HashMap<String, String>) -> Self;

    fn domain(self, domain: String) -> Self;

    fn discovery_options(self, options: DiscoveryOptions) -> Self;

    fn heartbeat_options(self, options: HeartbeatOptions) -> Self;

    fn build(self) -> R;
}
