use crate::clients::mdns::prelude::*;

#[derive(Default)]
struct MdnsContext {
    #[allow(dead_code)]
    service_name: String,
}

#[derive(Default, Clone)]
pub struct MdnRegistryOptions {
    pub service: IService,
    pub discovery: Option<MdnsDiscoveryOptions>,

    pub heartbeat: Option<HeartbeatOptions>,
}

pub struct MdnsServiceRegistry {
    registration: MdnsRegistration,
    #[allow(dead_code)]
    options: MdnRegistryOptions,
}

impl MdnsServiceRegistry {
    pub fn new(options: MdnRegistryOptions) -> MdnsServiceRegistry {
        let clone_option = options.clone();
        let heartbeat_option = options.heartbeat.expect("HeartbeatOptions is required");
        let mdns_discovery_option = options
            .discovery
            .expect("MdnsDiscoveryOptions is required.");

        let registration = MdnsRegistrationBuilder::new(None, None)
            .discovery_options(mdns_discovery_option)
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

        MdnsServiceRegistry {
            registration,
            options: clone_option,
        }
    }

    // fn generate_service(self) -> CatalogRegistration {
    //     self.registration.get_service()
    // }
}

#[async_trait]
impl ServiceRegistry for MdnsServiceRegistry {
    async fn register(&self) {
        let srv = self.registration.get_service().clone();
        info!(
            "registering service with id: {id}",
            id = srv.instance_id.clone()
        );

        // let mut txt = TxtRecord::new();
        // txt.insert("instance_id", srv.instance_id.clone().as_str());
        // txt.insert("node_id", srv.instance_id.as_str());
        // txt.insert("host", srv.host.as_str());
        // txt.insert("service_id", srv.service_name.as_str());
        // txt.insert("port", srv.port.to_string().as_str());
        // txt.insert("secure", srv.secure.to_string().as_str());

        let service =
            async_zeroconf::Service::new(srv.instance_id.as_str(), "_http._tcp", srv.port as u16);
        let service_ref = service.publish().await;
        match service_ref {
            Ok(_) => {
                info!(
                    "service with id: {id} registered",
                    id = srv.instance_id.clone()
                );
            }
            Err(_) => {
                info!(
                    "error service with id: {id} registered",
                    id = srv.instance_id.clone()
                );
            }
        };
    }

    async fn deregister(&self) {
        todo!()
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
