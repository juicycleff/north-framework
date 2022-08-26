use crate::clients::consul::prelude::*;

#[derive(Default, Clone)]
pub struct ConsulRegistryOptions {
    pub service: IService,
    pub discovery: Option<ConsulDiscoveryOptions>,

    pub heartbeat: Option<HeartbeatOptions>,
}

#[derive(Clone)]
pub struct ConsulServiceRegistry {
    consul: ConsulReactiveClient,
    registration: ConsulRegistration,
    options: ConsulRegistryOptions,
}

impl ConsulServiceRegistry {
    pub fn new(
        consul_option: ConsulConfig,
        options: ConsulRegistryOptions,
    ) -> ConsulServiceRegistry {
        let clone_option = options.clone();
        let heartbeat_option = options.heartbeat.expect("HeartbeatOptions is required");
        let consul_discovery_option = options
            .discovery
            .expect("ConsulDiscoveryOptions is required.");

        let registration = ConsulRegistrationBuilder::new(None, None)
            .discovery_options(consul_discovery_option.discovery_options)
            .heartbeat_options(heartbeat_option)
            .tags(options.service.tag.unwrap_or_default())
            .service_name(options.service.name)
            // .status(self.options.service.)
            // .version(self.registration.get_())
            .metadata(options.service.metadata.unwrap_or_default())
            .instance_id(options.service.id.clone())
            .host(options.service.host.clone())
            .port(options.service.port)
            .build();

        ConsulServiceRegistry {
            consul: ConsulReactiveClient::new_old(consul_option),
            registration,
            options: clone_option,
        }
    }

    // fn generate_service(self) -> CatalogRegistration {
    //     self.registration.get_service()
    // }
}

#[async_trait]
impl ServiceRegistry for ConsulServiceRegistry {
    async fn register(&self) {
        let reg = &self.registration.get_service();
        info!("registering service with id: {id}", id = reg.ID.clone());
        self.clone()
            .consul
            .client()
            .register(reg, None)
            .await
            .expect("error registering consul");
    }

    async fn deregister(&self) {
        self.clone()
            .consul
            .client()
            .deregister(
                &CatalogDeregistration {
                    Node: "".to_string(),
                    Address: "".to_string(),
                    Datacenter: "".to_string(),
                    ServiceID: self.registration.get_instance_id(),
                    CheckID: "".to_string(),
                },
                None,
            )
            .await
            .unwrap();
    }

    fn close(&self) {
        todo!()
    }

    fn set_status(&self, _status: String) {
        todo!()
    }

    // fn get_status<T: ServiceInstance>(&self) -> T {
    //     todo!()
    // }
}
