use crate::clients::consul::prelude::*;

pub type ConsulServiceInstance = DefaultServiceInstance;

/// Consul implementation of a discovery client
#[derive(Clone)]
pub struct ConsulDiscoveryClient {
    consul: ConsulReactiveClient,
}

/// ConsulDiscoveryClient members
impl ConsulDiscoveryClient {
    pub fn new(consul: ConsulReactiveClient) -> ConsulDiscoveryClient {
        ConsulDiscoveryClient { consul }
    }

    fn _find_host(self, health_service: &ServiceEntry) -> String {
        let service = &health_service.Service;
        let node = &health_service.Node;

        if !service.Address.is_empty() {
            return service.Address.clone();
        } else if !node.Address.is_empty() {
            return node.Address.clone();
        }

        node.Node.clone()
    }

    async fn add_instances_to_list(&self, service_id: &str) -> Vec<ConsulServiceInstance> {
        let (service_nodes, _meta) = self
            .clone()
            .consul
            .client()
            .service(service_id, None, true, None)
            .await
            .expect("Service not found");

        let mut result: Vec<ConsulServiceInstance> = vec![];
        for service_node in service_nodes {
            let host = if !service_node.Service.Address.is_empty() {
                service_node.Service.Address.clone()
            } else {
                service_node.Node.Address.clone()
            };

            let meta = service_node.Node.Meta.unwrap_or_default();
            let secure = match meta.clone().contains_key("secure") {
                true => meta.get("secure").unwrap().parse().unwrap(),
                false => false,
            };

            let status = service_node.Checks[0].Status.clone();

            let opts = ServiceInstanceOptions {
                instance_id: service_node.Service.ID.clone(),
                node_id: Some(service_node.Node.ID),
                service_id: service_node.Service.ID,
                host,
                status: status.into(),
                tags: service_node.Service.Tags,
                port: service_node.Service.Port as u32,
                secure,
                state: None,
                metadata: Some(meta),
            };

            result.push(ConsulServiceInstance::new(opts))
        }

        result
    }
}

/// Consul implementation of a discovery client DiscoveryClient<ConsulServiceInstance>
#[async_trait]
impl DiscoveryClient<ConsulServiceInstance> for ConsulDiscoveryClient {
    fn description(self) -> String {
        "ConsulClient Discovery Client".into()
    }

    async fn get_instances(self, service_id: String) -> Result<Vec<ConsulServiceInstance>, Error> {
        Ok(self.add_instances_to_list(service_id.as_str()).await)
    }

    async fn get_all_instances(self) -> Result<Vec<ConsulServiceInstance>, Error> {
        let services = self.clone().get_services().await.unwrap();

        let mut result: Vec<ConsulServiceInstance> = vec![];
        for service_id in services {
            let instance_list = self
                .clone()
                .add_instances_to_list(service_id.as_str())
                .await;
            for instance in instance_list {
                result.push(instance);
            }
        }

        Ok(result)
    }

    async fn get_services(self) -> Result<Vec<String>, Error> {
        let (service_nodes, _meta) = self
            .consul
            .client()
            .services(None)
            .await
            .expect("No services");
        let mut keys: Vec<String> = vec![];
        for x in service_nodes.keys() {
            keys.push(x.clone());
        }

        Ok(keys)
    }
}
