use crate::clients::consul::prelude::*;

#[derive(Default, Clone)]
pub struct ConsulDiscoveryOption {
    pub(crate) discovery_options: DiscoveryOptions,

    pub(crate) scheme: String,

    pub(crate) fail_fast: Option<bool>,

    pub(crate) notes: Option<String>,

    pub(crate) script: Option<String>,

    pub(crate) deregister_critical_service_after: Option<String>,
}

pub type ConsulDiscoveryOptions = ConsulDiscoveryOption;

#[derive(Default, Clone)]
pub struct ConsulRegistration {
    service: CatalogRegistration,
    discovery_options: ConsulDiscoveryOptions,
}

impl ConsulRegistration {
    /// creates a new instances of ConsulRegistration
    pub fn new(
        service: CatalogRegistration,
        discovery_options: ConsulDiscoveryOptions,
    ) -> ConsulRegistration {
        ConsulRegistration {
            service,
            discovery_options,
        }
    }
}

impl ServiceInstance for ConsulRegistration {
    fn get_instance_id(&self) -> String {
        self.service.ID.clone()
    }

    fn get_service_id(&self) -> String {
        self.service.ID.clone()
    }

    fn get_host(&self) -> String {
        self.service.Service.as_ref().unwrap().Address.clone()
    }

    fn get_port(&self) -> u32 {
        self.service.Service.as_ref().unwrap().Port as u32
    }

    fn is_secure(&self) -> bool {
        self.discovery_options.scheme == "https"
    }

    fn get_uri(&self) -> String {
        let port = self.get_port();
        let scheme = self.get_scheme();
        let host = self.get_host();

        format!(
            "{scheme}://{host}:{port}",
            scheme = scheme.as_str(),
            host = host,
            port = port
        )
    }

    fn get_scheme(&self) -> String {
        self.discovery_options.scheme.clone()
    }

    fn get_metadata(&self) -> HashMap<String, String> {
        self.service.NodeMeta.clone()
    }

    fn get_tags(&self) -> Vec<String> {
        let ch = self.service.Service.as_ref().unwrap();
        ch.clone().Tags.unwrap_or_default()
    }

    fn get_status(&self) -> String {
        let ch = self.service.Check.as_ref().unwrap();
        ch.Status.clone()
    }

    fn get_node_id(&self) -> String {
        self.service.Node.clone()
    }

    /// Not implemented for this API
    fn get_state(self) -> ServiceInstanceState {
        todo!()
    }
}

impl Registration<CatalogRegistration> for ConsulRegistration {
    fn get_service(&self) -> CatalogRegistration {
        self.service.clone()
    }
}
