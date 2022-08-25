use async_trait::async_trait;

use crate::registry::service_instance::ServiceInstance;
use crate::utils::server_utils::NorthResult;

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
    async fn get_instances(self, service_id: String) -> NorthResult<Vec<T>>;

    /// @returns all serviceInstances
    async fn get_all_instances(self) -> NorthResult<Vec<T>>;

    /// @returns all known services id
    async fn get_services(self) -> NorthResult<Vec<String>>;
}
