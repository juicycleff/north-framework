use async_trait::async_trait;
use crate::error::Error;

use crate::registry::service_instance::ServiceInstance;

/// #### DiscoveryClient
/// Base discovery client trait
#[async_trait]
pub trait DiscoveryClient<T>
where
    T: ServiceInstance,
{
    /// A human readable name to the implementation
    ///  @returns description of the client
    fn description(self) -> String;

    /// Gets all serviceInstance associated with the service id
    /// @param service_id name of the service to query
    /// @returns list of ServiceInstance
    async fn get_instances(self, service_id: String) -> Result<Vec<T>, Error>;

    /// @returns all serviceInstances
    async fn get_all_instances(self) -> Result<Vec<T>, Error>;

    /// @returns all known services id
    async fn get_services(self) -> Result<Vec<String>, Error>;
}
