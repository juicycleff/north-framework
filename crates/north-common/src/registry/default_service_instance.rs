use crate::registry::service_instance::{ServiceInstance, ServiceInstanceOptions};
use crate::registry::service_instance_state::ServiceInstanceState;
use std::collections::HashMap;

/// Base implementation of ServiceInstance
#[derive(Default, Clone)]
pub struct DefaultServiceInstance {
    state: ServiceInstanceState,
    options: ServiceInstanceOptions,
}

impl DefaultServiceInstance {
    /// instantiates a new DefaultServiceInstance
    pub fn new(opts: ServiceInstanceOptions) -> Self {
        match &opts.state {
            None => ServiceInstanceState::new(None),
            Some(o) => o.clone(),
        };

        let mut options = opts;
        let state = options.state.unwrap_or_default();
        options.state = Some(state.clone());

        DefaultServiceInstance { options, state }
    }
}

impl ServiceInstance for DefaultServiceInstance {
    fn get_instance_id(&self) -> String {
        self.options.instance_id.clone()
    }

    fn get_service_id(&self) -> String {
        self.options.service_id.clone()
    }

    fn get_host(&self) -> String {
        self.options.host.clone()
    }

    fn get_port(&self) -> u32 {
        self.options.port
    }

    fn is_secure(&self) -> bool {
        self.options.secure
    }

    fn get_uri(&self) -> String {
        let scheme = self.get_scheme();
        let host = self.get_host();
        let port = self.get_port();
        format!(
            "{scheme}://{host}:{port}",
            scheme = scheme.as_str(),
            host = host,
            port = port
        )
    }

    fn get_scheme(&self) -> String {
        match self.is_secure() {
            true => "https".to_string(),
            false => "http".to_string(),
        }
    }

    fn get_metadata(&self) -> HashMap<String, String> {
        let opt: HashMap<String, String> = HashMap::new();
        match self.options.clone().metadata {
            None => opt,
            Some(m) => m,
        }
    }

    fn get_tags(&self) -> Vec<String> {
        self.options.clone().tags.unwrap_or_default()
    }

    fn get_status(&self) -> String {
        self.options.status.to_string()
    }

    fn get_node_id(&self) -> String {
        self.options
            .clone()
            .node_id
            .unwrap_or_else(|| self.get_instance_id())
    }

    fn get_state(self) -> ServiceInstanceState {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use crate::registry::default_service_instance::DefaultServiceInstance;
    use crate::registry::service_instance::{ServiceInstance, ServiceInstanceOptions};
    use rstest::*;
    use std::collections::HashMap;

    #[fixture]
    fn service_options() -> ServiceInstanceOptions {
        let mut meta = HashMap::new();
        meta.insert(String::from("region"), "USA".to_string());

        ServiceInstanceOptions {
            instance_id: "test-service-1".to_string(),
            node_id: Some("test-node-1".to_string()),
            service_id: "test-service".to_string(),
            host: "127.0.5.0".to_string(),
            status: Default::default(),
            tags: Some(vec![String::from("service"), String::from("rust")]),
            port: 8080,
            secure: false,
            state: None,
            metadata: Some(meta),
        }
    }

    #[fixture]
    fn secure_service_options() -> ServiceInstanceOptions {
        ServiceInstanceOptions {
            instance_id: "test-service-1".to_string(),
            node_id: Some("test-node-1".to_string()),
            service_id: "test-service".to_string(),
            host: "127.0.5.0".to_string(),
            status: Default::default(),
            tags: None,
            port: 8080,
            secure: true,
            state: None,
            metadata: None,
        }
    }

    #[fixture]
    fn service() -> DefaultServiceInstance {
        DefaultServiceInstance::new(service_options())
    }

    #[fixture]
    fn secure_service() -> DefaultServiceInstance {
        DefaultServiceInstance::new(secure_service_options())
    }

    #[rstest]
    fn it_can_get_service_host(service: DefaultServiceInstance) {
        assert_eq!(service.get_host(), service_options().host);
    }

    #[rstest]
    fn it_can_get_service_port(service: DefaultServiceInstance) {
        assert_eq!(service.get_port(), service_options().port);
    }

    #[rstest]
    fn it_can_check_service_secure(service: DefaultServiceInstance) {
        assert_eq!(service.is_secure(), service_options().secure);
    }

    #[rstest]
    #[case::secure(secure_service())]
    #[case::insecure(service())]
    fn it_can_check_service_scheme(#[case] service: DefaultServiceInstance) {
        match service.is_secure() {
            true => {
                assert_eq!(service.get_scheme(), "https");
            }
            false => {
                assert_eq!(service.get_scheme(), "http");
            }
        }
    }

    #[rstest]
    #[case::secure(secure_service())]
    #[case::insecure(service())]
    fn it_can_check_service_uri(#[case] service: DefaultServiceInstance) {
        match service.is_secure() {
            true => {
                assert_eq!(
                    service.get_uri(),
                    format!(
                        "https://{host}:{port}",
                        host = secure_service_options().host,
                        port = secure_service_options().port
                    )
                );
            }
            false => {
                assert_eq!(
                    service.get_uri(),
                    format!(
                        "http://{host}:{port}",
                        host = service_options().host,
                        port = service_options().port
                    )
                );
            }
        }
    }

    #[rstest]
    fn it_can_get_service_node_id(service: DefaultServiceInstance) {
        assert_eq!(
            service.get_node_id(),
            service_options().node_id.unwrap_or_default()
        );
    }

    #[rstest]
    fn it_can_get_service_service_id(service: DefaultServiceInstance) {
        assert_eq!(service.get_service_id(), service_options().service_id);
    }

    #[rstest]
    fn it_can_get_service_instance_id(service: DefaultServiceInstance) {
        assert_eq!(service.get_instance_id(), service_options().instance_id);
    }

    #[rstest]
    fn it_can_get_service_tags(service: DefaultServiceInstance) {
        assert_eq!(
            service.get_tags(),
            service_options().tags.unwrap_or_default()
        );
        assert_eq!(service.get_tags().len(), 2);
    }

    #[rstest]
    fn it_can_get_service_metadata(service: DefaultServiceInstance) {
        assert!(service.get_metadata().contains_key("region"));
        assert_eq!(
            service.get_metadata(),
            service_options().metadata.unwrap_or_default()
        );
    }
}
