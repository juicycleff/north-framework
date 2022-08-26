use crate::clients::mdns::prelude::*;

pub type MdnsDiscoveryOptions = DiscoveryOptions;

#[derive(Default, Debug, Clone)]
pub struct MdnsContext {
    pub service_name: String,
    pub node_id: String,
    pub service_id: String,
    pub instance_id: String,
    pub host: String,
    pub status: String,
    pub port: u32,
    pub secure: bool,
    pub metadata: Option<HashMap<String, String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Default, Clone)]
pub struct MdnsRegistration {
    service: MdnsContext,
    discovery_options: MdnsDiscoveryOptions,
}

impl MdnsRegistration {
    /// creates a new instances of MdnsRegistration
    pub fn new(service: MdnsContext, discovery_options: MdnsDiscoveryOptions) -> MdnsRegistration {
        MdnsRegistration {
            service,
            discovery_options,
        }
    }
}

impl ServiceInstance for MdnsRegistration {
    fn get_instance_id(&self) -> String {
        self.service.instance_id.clone()
    }

    fn get_service_id(&self) -> String {
        self.service.service_id.clone()
    }

    fn get_host(&self) -> String {
        self.service.host.clone()
    }

    fn get_port(&self) -> u32 {
        self.service.port
    }

    fn is_secure(&self) -> bool {
        self.service.secure
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
        self.service.clone().metadata.unwrap_or_default()
    }

    fn get_tags(&self) -> Vec<String> {
        self.service.clone().tags.unwrap_or_default()
    }

    fn get_status(&self) -> String {
        self.service.status.clone()
    }

    fn get_node_id(&self) -> String {
        self.service.node_id.clone()
    }

    /// Not implemented for this API
    fn get_state(self) -> ServiceInstanceState {
        todo!()
    }
}

impl Registration<MdnsContext> for MdnsRegistration {
    fn get_service(&self) -> MdnsContext {
        self.service.clone()
    }
}
